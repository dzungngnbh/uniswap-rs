use alloy::primitives::U256;

/// Divides two U256 numbers and rounds up the result.
///
/// safety: This function does not check input and output values for overflow.
pub fn unsafe_div_round_up(x: U256, y: U256) -> U256 {
    let quotient = x.wrapping_div(y);
    let remainder = x.wrapping_rem(y);
    if remainder.is_zero() {
        quotient
    } else {
        quotient + U256::from(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div_round_up() {
        assert_eq!(
            unsafe_div_round_up(U256::from(1), U256::from(1)),
            U256::from(1)
        );
        assert_eq!(
            unsafe_div_round_up(U256::from(1), U256::from(2)),
            U256::from(1)
        );
        assert_eq!(
            unsafe_div_round_up(U256::from(2), U256::from(2)),
            U256::from(1)
        );
    }
}
