//! Module to create random numbers

#[cfg(feature = "random_numbers")]
pub mod random_numbers {
    use rand::Rng;
    pub fn get_random_number_range(min_num: f64, max_num: f64) -> f64 {
        return rand::thread_rng().gen_range(min_num..max_num);
    }
}

#[cfg(feature = "random_numbers")]
#[cfg(test)]
mod test {
    #[test]
    fn test_rand_num_between_1_10() {
        let test_range = 1.0..10.0;
        let num = super::random_numbers::get_random_number_range(1.0, 10.0);
        assert!(test_range.contains(&num));
    }
    #[test]
    fn test_rand_num_between_1_100() {
        let test_range = 1.0..100.0;
        let num = super::random_numbers::get_random_number_range(1.0, 100.0);
        assert!(test_range.contains(&num));
    }
    #[test]
    fn test_rand_num_between_1_1000() {
        let test_range = 1.0..1000.0;
        let num = super::random_numbers::get_random_number_range(1.0, 1000.0);
        assert!(test_range.contains(&num));
    }
}