pub trait AppNumber {
    fn is_valid(&self) -> bool;
}

impl AppNumber for u16 {
    fn is_valid(&self) -> bool {
        self > &0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_is_valid_when_it_is_greater_than_zero() {
        assert!(1.is_valid());
    }

    #[test]
    fn it_is_not_valid_when_it_is_equal_to_zero() {
        assert!(!0.is_valid());
    }
}
