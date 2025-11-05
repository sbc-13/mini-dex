use anchor_lang::prelude::*;
use crate::errors::DexError;

/// Calcula el output de un swap usando constant product formula
/// Formula: (reserve_out * amount_in * (FEE_DEN - fee_bps)) / (reserve_in * FEE_DEN + amount_in * (FEE_DEN - fee_bps))
/// Donde `fee_bps` está en basis points (p. ej., 30 = 0.3%) y `FEE_DEN = 10_000`.
pub fn calculate_swap_amount(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_bps: u64,
) -> Result<u64> {
    require!(amount_in > 0, DexError::InvalidCalculation);
    require!(reserve_in > 0 && reserve_out > 0, DexError::InsufficientLiquidity);
    require!(fee_bps <= 10_000, DexError::InvalidCalculation);

    const FEE_DEN: u128 = 10_000;
    let fee_multiplier: u128 = (10_000u128)
        .checked_sub(fee_bps as u128)
        .ok_or(DexError::MathOverflow)?;

    // Aplicar fee configurable en bps
    let amount_in_with_fee = (amount_in as u128)
        .checked_mul(fee_multiplier)
        .ok_or(DexError::MathOverflow)?;

    let numerator = amount_in_with_fee
        .checked_mul(reserve_out as u128)
        .ok_or(DexError::MathOverflow)?;

    let denominator = (reserve_in as u128)
        .checked_mul(FEE_DEN)
        .ok_or(DexError::MathOverflow)?
        .checked_add(amount_in_with_fee)
        .ok_or(DexError::MathOverflow)?;

    let amount_out = numerator
        .checked_div(denominator)
        .ok_or(DexError::InvalidCalculation)?;

    Ok(amount_out as u64)
}

/// Calcula la cantidad de LP tokens a mintear cuando se añade liquidez
pub fn calculate_lp_tokens(
    amount_a: u64,
    amount_b: u64,
    reserve_a: u64,
    reserve_b: u64,
    lp_supply: u64,
) -> Result<u64> {
    if lp_supply == 0 {
        // Primera vez añadiendo liquidez: sqrt(a * b)
        let product = (amount_a as u128)
            .checked_mul(amount_b as u128)
            .ok_or(DexError::MathOverflow)?;

        // Aproximación simple de square root
        Ok((product as f64).sqrt() as u64)
    } else {
        // Liquidez subsecuente: mantener ratio
        let lp_a = ((amount_a as u128)
            .checked_mul(lp_supply as u128)
            .ok_or(DexError::MathOverflow)?)
            .checked_div(reserve_a as u128)
            .ok_or(DexError::InvalidCalculation)?;

        let lp_b = ((amount_b as u128)
            .checked_mul(lp_supply as u128)
            .ok_or(DexError::MathOverflow)?)
            .checked_div(reserve_b as u128)
            .ok_or(DexError::InvalidCalculation)?;

        // Tomar el mínimo para prevenir exploits
        Ok(std::cmp::min(lp_a, lp_b) as u64)
    }
}