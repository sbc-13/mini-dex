
use anchor_lang::prelude::*;

/// Pool account - Guarda el estado del liquidity pool
#[account]
pub struct Pool {
    /// Authority PDA que controla el pool
    pub authority: Pubkey,

    /// Mint address del Token A (ej: SOL)
    pub token_a_mint: Pubkey,

    /// Mint address del Token B (ej: USDC)
    pub token_b_mint: Pubkey,

    /// Vault que guarda Token A
    pub token_a_vault: Pubkey,

    /// Vault que guarda Token B
    pub token_b_vault: Pubkey,

    /// LP Token Mint
    pub lp_token_mint: Pubkey,

    /// Reserves de Token A en el pool
    pub reserve_a: u64,

    /// Reserves de Token B en el pool
    pub reserve_b: u64,

    /// Fee en basis points (30 = 0.3%)
    pub fee_numerator: u64,

    /// Bump seed para el authority PDA
    pub authority_bump: u8,
}

impl Pool {
    /// Tamaño en bytes de la cuenta Pool
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // token_a_mint
        32 + // token_b_mint
        32 + // token_a_vault
        32 + // token_b_vault
        32 + // lp_token_mint
        8 +  // reserve_a
        8 +  // reserve_b
        8 +  // fee_numerator
        1;   // authority_bump
}