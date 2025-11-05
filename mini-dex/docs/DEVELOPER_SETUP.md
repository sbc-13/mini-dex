### Mini-DEX — Developer Environment Setup (Junior-Friendly)

This guide helps you clone, build, deploy, and test Mini‑DEX locally. It’s written for beginners to Solana/Anchor with step‑by‑step instructions and troubleshooting tips — especially for macOS.

#### Prerequisites
- Git and a terminal (macOS, Linux, or WSL on Windows)
- Rust toolchain
- Solana CLI
- Anchor CLI
- Node.js + yarn
- jq (optional, for pretty JSON)
- Docker (optional, for alternate validator)

Suggested versions (as of 2025‑11‑05):
- Solana CLI: 1.18+ or 2.x/3.x (Anchor localnet works with 1.18+; your logs showed 3.0.10 during localnet)
- Anchor CLI: 0.29+
- Node: >= 20.18.0 (LTS)
- Yarn: 1.x (classic)

---

### 1) Install toolchains

macOS (arm64/Intel) and Linux:

1) Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version
```

2) Solana CLI
```
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
solana --version
```
If you had Solana before, run `solana-install update` to get the latest stable.

3) Anchor CLI
```
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
anchor --version
```
If you prefer crates.io:
```
cargo install --locked anchor-cli
```

4) Node + yarn
- Install Node via nvm or your OS package manager
```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
source ~/.nvm/nvm.sh
nvm install --lts
node -v
npm i -g yarn@1
yarn -v
```

Optional tools:
```
brew install jq       # macOS (Homebrew)
sudo apt-get install jq -y   # Ubuntu/Debian
```

Windows users:
- Use WSL (Ubuntu) for the best experience, then follow Linux steps.

---

### 2) Clone the repository
```
git clone <YOUR_GITHUB_URL>.git
cd ProjectWeek/mini-dex
```

Note for macOS cloud-synced folders (OneDrive/iCloud/Dropbox):
- Prefer a local non‑synced path (e.g., `~/code/mini-dex`) to avoid macOS resource‑fork files (`._*`) interfering with the validator.

---

### 3) Install JS dependencies
Do not mix package managers. If `package-lock.json` exists (from npm), delete it before using Yarn classic:
```
rm -f package-lock.json
```
Now install dependencies:
```
yarn install
```

---

### 4) Configure Solana for local development
Always use IPv4 to avoid `::1` (IPv6) connection issues on macOS.
```
solana config set --url http://127.0.0.1:8899
solana config get
```
Your wallet defaults to `~/.config/solana/id.json`. If it doesn’t exist:
```
solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
```

---

### 5) Start a local validator (Anchor-managed)
Use Anchor’s built-in localnet in Terminal 1. On macOS, disable AppleDouble files to prevent genesis unpack errors.
```
# Terminal 1
COPYFILE_DISABLE=1 anchor localnet
```
You should see lines like:
```
JSON RPC URL: http://127.0.0.1:8899
WebSocket PubSub URL: ws://127.0.0.1:8900
```

Verify RPC is up (optional, Terminal 2):
```
curl -s http://127.0.0.1:8899 | jq .
```

---

### 6) Build, deploy, and run tests
Open Terminal 2 in the `mini-dex` folder.

1) Set env vars (use IPv4 explicitly):
```
export ANCHOR_PROVIDER_URL="http://127.0.0.1:8899"
export ANCHOR_WALLET="$HOME/.config/solana/id.json"
```

2) Build the on-chain program:
```
anchor build
```

3) Deploy program to localnet:
```
anchor deploy
```
You should see a Program Id printed (e.g., `HazKKeR...`). `Anchor.toml` keeps it under `[programs.localnet]`.

4) Run tests:
```
yarn test
```
You should get 5 passing tests covering: initialize pool, add liquidity, swap (A→B and B→A), remove liquidity.

---

### 7) Common pitfalls and fixes (macOS heavy)

1) Genesis unpack error mentioning `._genesis.bin`
- Symptom:
```
Archive error: extra entry found: "._genesis.bin"
```
- Cause: macOS resource-fork files created in synced folders.
- Fixes:
  - Run localnet with `COPYFILE_DISABLE=1`.
  - Prefer ledger outside synced folders.
  - Use Anchor localnet (it uses `.anchor/test-ledger`).

2) IPv6 `ECONNREFUSED ::1:8899`
- Symptom: Tests/readiness checks fail with IPv6 address.
- Fix: Always use IPv4 `http://127.0.0.1:8899` for `solana config` and `ANCHOR_PROVIDER_URL`.

3) Upgrade authority mismatch on deploy
- Symptom:
```
Program's authority Some(11111111111111111111111111111111) does not match authority provided <your_key>
```
- Cause: Old program ID in the current ledger is immutable (authority = `111111...`).
- Fix options:
  - Wipe Anchor ledger and restart:
    ``
    pkill -f anchor-localnet || true
    rm -rf .anchor/test-ledger
    COPYFILE_DISABLE=1 anchor localnet
    ``
  - Or generate a new program ID and sync:
    ``
    solana-keygen new -o target/deploy/mini_dex-keypair.json --no-bip39-passphrase -f
    anchor keys sync
    anchor build && anchor deploy
    ```

4) Permission denied removing `.anchor/test-ledger`
- Use:
```
sudo rm -rf .anchor/test-ledger
```
Then restart localnet.

5) Node ESM warning from ts-mocha
- Symptom: `MODULE_TYPELESS_PACKAGE_JSON`
- Optional fix: add to `mini-dex/package.json`:
```
{
  "type": "module"
}
```

6) Dockerized validator (fully isolated)
If host issues persist, run validator in Docker:
```
docker run --rm -it \
  -p 8899:8899 -p 8900:8900 \
  --name solana-localnet \
  solanalabs/solana:stable \
  solana-test-validator --reset --limit-ledger-size
```
Then in your shell:
```
export ANCHOR_PROVIDER_URL="http://127.0.0.1:8899"
solana config set --url http://127.0.0.1:8899
anchor build && anchor deploy && yarn test
```

---

### 8) Useful commands
```
# Display versions
solana --version && anchor --version && node -v && yarn -v

# Show current Solana config
solana config get

# List program IDs known to Anchor
anchor keys list

# Show on-chain info for a program
solana program show <PROGRAM_ID>

# Airdrop SOL to your keypair on localnet
solana airdrop 2 $(solana address)
```

---

### 9) Directory quick map (recap)
- `programs/mini-dex/src/lib.rs` — instructions
- `programs/mini-dex/src/state.rs` — account state (`Pool`)
- `programs/mini-dex/src/utils.rs` — math helpers
- `programs/mini-dex/src/errors.rs` — error types
- `tests/mini-dex.ts` — integration tests
- `Anchor.toml` — program IDs, scripts, provider config

You’re ready to build and experiment. Happy hacking!