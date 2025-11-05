### Mini-DEX — Executive Summary

#### What is Mini-DEX?
Mini-DEX is a minimal, educational Automated Market Maker (AMM) decentralized exchange built on Solana using Anchor. It demonstrates the core mechanics behind popular constant-product AMMs: pool initialization, adding/removing liquidity, and token swaps between two SPL tokens. The project is intentionally compact to help newcomers understand the building blocks of on-chain liquidity protocols on Solana.

#### Problem & Opportunity
Traditional order-book exchanges are complex and capital‑intensive. AMMs enable continuous liquidity for long-tail assets with a simple, transparent pricing function (x*y = k). On Solana, low latency and high throughput make AMMs particularly powerful, but the learning curve for building them is steep. Mini-DEX serves as an approachable reference implementation.

#### What it does (today)
- Initializes a two‑token liquidity pool (SPL Token A and B)
- Mints LP tokens to liquidity providers proportionally to deposits
- Executes swaps in either direction using constant‑product pricing with fees
- Allows LPs to remove liquidity and redeem underlying tokens
- Provides a complete test suite in TypeScript to exercise all instructions

#### Technology Stack
- Blockchain: Solana
- Smart contracts: Rust with Anchor framework
- Token standard: SPL Token
- Program architecture: PDA‑secured vaults and LP mint
- Client/tests: TypeScript, `@coral-xyz/anchor`, `@solana/web3.js`, `@solana/spl-token`, `ts-mocha`
- Local development: `anchor localnet` (Solana test validator), configurable fees in basis points

#### How it works (high level)
- Constant‑product AMM: Pricing follows x*y = k with a swap fee (default 30 bps). Fees are applied to input amounts to keep the invariant stable.
- Liquidity provisioning: First deposit sets pool price; subsequent deposits must respect the current ratio to mint LP tokens fairly.
- Security model: Program Derived Address (PDA) owns the token vaults and LP token mint; account constraints bind instruction inputs to the pool’s recorded addresses and mints.

#### Differentiation vs. Market Players
Mini-DEX is not meant to compete in production; it’s a learning scaffold. For context:
- Uniswap (EVM): Popularized constant‑product AMMs; later introduced concentrated liquidity (v3/v4). Not on Solana.
- Raydium (Solana): AMM + order book integration with OpenBook, deeper liquidity and routing.
- Orca (Solana): User‑friendly AMM with various pools and fees, Whirlpool concentrated liquidity.
- Meteora (Solana): Dynamic liquidity market makers, adaptive pools, liquidity incentives.
- Phoenix/Jupiter (Solana): Phoenix is a CLOB; Jupiter is a router/aggregator across venues.

Mini-DEX focuses on clarity over features: a small, auditable codebase you can read in one sitting.

#### Intended Users
- Web3 learners and junior smart contract developers
- Educators and bootcamps teaching Solana/Anchor
- Teams prototyping AMM ideas before adding advanced features (CLMM, multi‑token, routing)

#### Roadmap Ideas (optional)
- Configurable governance for fee updates
- Concentrated liquidity positions
- TWAP oracles and price bounds
- Router support for best‑execution against multiple pools
- Cross‑program invocations (CPI) for composability with other DeFi primitives

#### Risks & Considerations
- Educational only; unaudited code
- No production‑grade risk management (oracle checks, sandwich protection, etc.)
- Floating‑point approximation in initial LP mint calculation (acceptable for demos; replace with integer sqrt for production)

#### Licensing & Community
- Open source learning resource
- Contributions welcome via pull requests and issues

