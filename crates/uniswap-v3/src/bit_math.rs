use alloy::primitives::U256;
use anyhow::Result;

/// Returns the index of the most significant bit in the given U256 number.
pub fn most_significant_bit(x: U256) -> Result<u8> {
    if !x.is_zero() {
        return Ok(255 - x.leading_zeros() as u8);
    }

    Err(anyhow::anyhow!("zero has no most significant bit"))
}

/// Returns the index of the least significant bit in the given U256 number.
pub fn least_significant_bit(x: U256) -> Result<u8> {
    if !x.is_zero() {
        return Ok(x.trailing_zeros() as u8);
    }

    Err(anyhow::anyhow!("zero has no least significant bit"))
}

#[cfg(test)]
mod tests {
    use super::*;

    // test cases from the uniswap v3 codebase
    #[test]
    fn test_most_significant_bit() {
        assert!(most_significant_bit(U256::ZERO).is_err());

        // 1
        let res = most_significant_bit(U256::from(1));
        assert_eq!(res.unwrap(), 0);

        // 2
        let res = most_significant_bit(U256::from(2));
        assert_eq!(res.unwrap(), 1);

        // power of 2
        for i in 0..256 {
            let res = most_significant_bit(U256::from(1) << i);
            assert_eq!(res.unwrap(), i as u8);
        }

        // uint256(-1)
        let res = most_significant_bit(U256::MAX);
        assert_eq!(res.unwrap(), 255);
    }

    #[test]
    fn test_least_significant_bit() {
        assert!(least_significant_bit(U256::ZERO).is_err());

        // 1
        let res = least_significant_bit(U256::from(1));
        assert_eq!(res.unwrap(), 0);

        // 2
        let res = least_significant_bit(U256::from(2));
        assert_eq!(res.unwrap(), 1);

        // power of 2
        for i in 0..256 {
            let res = least_significant_bit(U256::from(1) << i);
            assert_eq!(res.unwrap(), i as u8);
        }

        // uint256(-1)
        let res = least_significant_bit(U256::MAX);
        assert_eq!(res.unwrap(), 0);
    }
}
