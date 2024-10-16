use alloy::primitives::U256;
use anyhow::Result;
use std::ops::{Add, BitAnd, BitOrAssign, BitXor, Div, Mul, MulAssign};

/// Calculates floor(a*b/denominator) with full precision.
///
/// Throws if result overflows a uint256 or denominator == 0
pub fn mul_div(x: U256, y: U256, mut denominator: U256) -> Result<U256> {
    // 512-bit multiply [prod1 prod0] = a * b
    // Compute the product mod 2**256 and mod 2**256 - 1
    // then use the Chinese Remainder Theorem to reconstruct
    // the 512 bit result. The result is stored in two 256
    // variables such that product = prod1 * 2**256 + prod0
    let mm = x.mul_mod(y, U256::MAX);

    // least significant 256 bits of the product
    let mut prod0 = x.overflowing_mul(y).0;
    // most significant 256 bits of the product
    let mut prod1 = mm
        .overflowing_sub(prod0)
        .0
        .overflowing_sub(U256::from((mm < prod0) as u8))
        .0;

    // Handle non-overflow cases, 256 by 256 division
    if prod1.is_zero() {
        if denominator.is_zero() {
            return Err(anyhow::anyhow!("division by zero"));
        }
        return Ok(U256::from_limbs(*prod0.div(denominator).as_limbs()));
    }

    if prod1 >= denominator {
        return Err(anyhow::anyhow!("Prod1 > denominator"));
    }

    let remainder = x.mul_mod(y, denominator);

    // subtract 256 bit number from 512 bit number
    prod1 = prod1
        .overflowing_sub(U256::from((remainder > prod0) as u8))
        .0;
    prod0 = prod0.overflowing_sub(remainder).0;

    // Factor powers of two out of denominator
    // Compute largest power of two divisor of denominator.
    // Always >= 1.
    let mut twos = (-denominator).bitand(denominator);

    // Divide denominator by power of two
    denominator = denominator.wrapping_div(twos);

    // Divide [prod1 prod0] by the factors of two
    prod0 = prod0.wrapping_div(twos);

    // Shift in bits from prod1 into prod0. For this we need
    // to flip `twos` such that it is 2**256 / twos.
    // If twos is zero, then it becomes one
    twos = U256::ZERO
        .overflowing_sub(twos)
        .0
        .wrapping_div(twos)
        .add(U256::from(1));

    prod0.bitor_assign(prod1 * twos);

    // Invert denominator mod 2**256
    // Now that denominator is an odd number, it has an inverse
    // modulo 2**256 such that denominator * inv = 1 mod 2**256.
    // Compute the inverse by starting with a seed that is correct
    // correct for four bits. That is, denominator * inv = 1 mod 2**4
    let three = U256::from(3);
    let two = U256::from(2);
    let mut inv = three.mul(denominator).bitxor(two);

    // Now use Newton-Raphson iteration to improve the precision.
    // Thanks to Hensel's lifting lemma, this also works in modular
    // arithmetic, doubling the correct bits in each step.
    inv.mul_assign(two - denominator * inv); // inverse mod 2**8
    inv.mul_assign(two - denominator * inv); // inverse mod 2**16
    inv.mul_assign(two - denominator * inv); // inverse mod 2**32
    inv.mul_assign(two - denominator * inv); // inverse mod 2**64
    inv.mul_assign(two - denominator * inv); // inverse mod 2**128
    inv.mul_assign(two - denominator * inv); // inverse mod 2**256

    // Because the division is now exact we can divide by multiplying
    // with the modular inverse of denominator. This will give us the
    // correct result modulo 2**256. Since the preconditions guarantee
    // that the outcome is less than 2**256, this is the final result.
    // We don't need to compute the high bits of the result and prod1
    // is no longer required.
    Ok(U256::from_le_slice((prod0 * inv).as_le_slice()))
}

pub fn mul_div_round_up(x: U256, y: U256, denominator: U256) -> Result<U256> {
    let result = mul_div(x, y, denominator)?;
    if x.mul_mod(y, denominator) > U256::ZERO {
        if result == U256::MAX {
            Err(anyhow::anyhow!("Result is MAX"))
        } else {
            Ok(result + U256::from(1))
        }
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const Q128: U256 = U256::from_limbs([0, 0, 1, 0]);

    #[test]
    fn test_mul_div() {
        // Accurate with phantom overflow
        let result = mul_div(Q128, U256::from(35).mul(Q128), U256::from(8).mul(Q128));
        assert_eq!(
            result.unwrap(),
            U256::from(4375).mul(Q128).div(U256::from(1000))
        );

        // Accurate with phantom overflow
        let result = mul_div(Q128, U256::from(35).mul(Q128), U256::from(8).mul(Q128));
        assert_eq!(
            result.unwrap(),
            U256::from(4375).mul(Q128).div(U256::from(1000))
        );

        // Accurate with phantom overflow and repeating decimal
        // let result = mul_div(Q18, U256::from(1000).mul(Q128), U256::from(3000).mul(Q128));
        // assert_eq!(result.unwrap(), Q128.div(U256::from(3)));

        // Error if the denominator is zero
        let result = mul_div(Q128, U256::from(1), U256::ZERO);
        assert!(result.is_err());

        // Error if the result overflows
        let result = mul_div(Q128, Q128, U256::from(1));
        assert!(result.is_err());

        // Error if denominator is zero and result overflows
        let result = mul_div(Q128, Q128, U256::ZERO);
        assert!(result.is_err());

        // Error on overflow with max inputs
        let result = mul_div(U256::MAX, U256::MAX, U256::from(1));
        assert!(result.is_err());

        // Error with all max inputs
        // let result = mul_div(U256::MAX, U256::MAX, U256::MAX);
        // assert!(result.is_err());
    }
}
