use anyhow::Result;

/// Adds a signed integer to an unsigned integer.
///
/// safety: This function check overflow and returns an error if it occurs.
pub fn add_delta(x: u128, y: i128) -> Result<u128> {
    if y < 0 {
        match x.checked_sub(y.unsigned_abs()) {
            Some(z) => Ok(z),
            None => Err(anyhow::anyhow!("overflow")),
        }
    } else {
        match x.checked_add(y as u128) {
            Some(z) => Ok(z),
            None => Err(anyhow::anyhow!("overflow")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_delta() {
        assert_eq!(add_delta(1, 1).unwrap(), 2);
        assert_eq!(add_delta(1, 0).unwrap(), 1);
        assert_eq!(add_delta(1, -1).unwrap(), 0);
        assert!(add_delta(u128::MAX, 1).is_err());
        assert!(add_delta(0, -1).is_err());
    }
}
