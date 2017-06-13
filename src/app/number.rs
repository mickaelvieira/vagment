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
    fn it_knows_when_it_is_a_valid_machine_number() {
        assert!(1.is_valid());
        assert!(!0.is_valid());
    }
}
