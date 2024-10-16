use alloy::primitives::U256;
use anyhow::Result;

use crate::full_math::mul_div;
use crate::{full_math::mul_div_round_up, unsafe_math::unsafe_div_round_up};

pub const MAX_U160: U256 =
    U256::from_limbs([18446744073709551615, 18446744073709551615, 4294967295, 0]);
pub const Q96: U256 = U256::from_limbs([0, 4294967296, 0, 0]);
pub const FIXED_POINT_96_RESOLUTION: U256 = U256::from_limbs([96, 0, 0, 0]);

/// @notice Gets the next sqrt price given a delta of token0
/// @dev Always rounds up, because in the exact output case (increasing price) we need to move the price at least
/// far enough to get the desired output amount, and in the exact input case (decreasing price) we need to move the
/// price less in order to not send too much output.
/// The most precise formula for this is liquidity * sqrtPX96 / (liquidity +- amount * sqrtPX96),
/// if this is impossible because of overflow, we calculate liquidity / (liquidity / sqrtPX96 +- amount).
/// @param sqrtPX96 The starting price, i.e. before accounting for the token0 delta
/// @param liquidity The amount of usable liquidity
/// @param amount How much of token0 to add or remove from virtual reserves
/// @param add Whether to add or remove the amount of token0
/// @return The price after adding or removing amount, depending on add
pub fn get_next_sqrt_price_from_amount0_rounding_up(
    sqrt_price_x_96: U256,
    liquidity: u128,
    amount: U256,
    add: bool,
) -> Result<U256> {
    if amount.is_zero() {
        // perf: unlikely
        return Ok(sqrt_price_x_96);
    }

    let numerator1: U256 = U256::from(liquidity) << 96;

    if add {
        let product = amount.wrapping_mul(sqrt_price_x_96);
        if product.wrapping_div(amount) == sqrt_price_x_96 {
            let denominator = numerator1.wrapping_add(product);

            if denominator >= numerator1 {
                return mul_div_round_up(numerator1, sqrt_price_x_96, denominator);
            }
        }

        Ok(unsafe_div_round_up(
            numerator1,
            (numerator1.wrapping_div(sqrt_price_x_96)).wrapping_add(amount),
        ))
    } else {
        let product = amount.wrapping_mul(sqrt_price_x_96);
        if product.wrapping_div(amount) == sqrt_price_x_96 && numerator1 > product {
            let denominator = numerator1.wrapping_sub(product);

            mul_div_round_up(numerator1, sqrt_price_x_96, denominator)
        } else {
            Err(anyhow::anyhow!("Product div amount"))
        }
    }
}

/// Gets the next sqrt price given a delta of token1
/// @dev Always rounds down, because in the exact output case (decreasing price) we need to move the price at least
/// far enough to get the desired output amount, and in the exact input case (increasing price) we need to move the
/// price less in order to not send too much output.
/// The formula we compute is within <1 wei of the lossless version: sqrtPX96 +- amount / liquidity
///
/// Returns uint160 sqrtQX96
pub fn get_next_sqrt_price_from_amount1_rounding_down(
    sqrt_price_x_96: U256,
    liquidity: u128,
    amount: U256,
    add: bool,
) -> Result<U256> {
    let liq = U256::from(liquidity);

    if add {
        let quotient = if amount <= MAX_U160 {
            (amount << FIXED_POINT_96_RESOLUTION) / liq
        } else {
            mul_div(amount, Q96, liq)?
        };

        let new_sqrt_price_x_96 = quotient + sqrt_price_x_96;
        if new_sqrt_price_x_96 > MAX_U160 {
            return Err(anyhow::anyhow!("new_sqrt_price_x_96 > MAX_U160"));
        }

        Ok(new_sqrt_price_x_96)
    } else {
        let quotient = if amount <= MAX_U160 {
            unsafe_div_round_up(amount << FIXED_POINT_96_RESOLUTION, liq)
        } else {
            mul_div_round_up(amount, Q96, liq)?
        };

        if quotient >= sqrt_price_x_96 {
            return Err(anyhow::anyhow!("quotient >= sqrt_price_x_96"));
        }

        Ok(sqrt_price_x_96.overflowing_sub(quotient).0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {}
}
