use anchor_lang::prelude::*;

#[error_code]
pub enum DexError {
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[msg("Invalid calculation - would result in zero output")]
    InvalidCalculation,

    #[msg("Insufficient liquidity in pool")]
    InsufficientLiquidity,

    #[msg("Math overflow occurred")]
    MathOverflow,

    #[msg("Invalid token mints - must be different")]
    InvalidTokenMints,
}