//! This is a collection of bio algorithms
pub mod bio_algorithms {
    /// This function calculates the hemming distance of two strings
    ///
    /// <https://docs.rs/bio/latest/bio/alignment/distance/fn.hamming.html>
    pub fn hamming(alpha_str: String, beta_str: String) -> u64 {
        return bio::alignment::distance::hamming(alpha_str.as_bytes(), beta_str.as_bytes());
    }

    /// This function calculates the levenshtein distance of two strings
    /// 
    /// <https://docs.rs/bio/latest/bio/alignment/distance/fn.levenshtein.html>
    pub fn levenshtein(alpha_str: String, beta_str: String) -> u32 {
        return bio::alignment::distance::levenshtein(alpha_str.as_bytes(), beta_str.as_bytes());
    }
    /// This calculates the SIMD-accelerated Levenshtein
    /// 
    /// <https://docs.rs/bio/latest/bio/alignment/distance/simd/fn.levenshtein.html>
    pub fn levenshtein_smid(alpha_str: String, beta_str: String) -> u32 {
        return bio::alignment::distance::simd::levenshtein(
            alpha_str.as_bytes(),
            beta_str.as_bytes(),
        );
    }
    /// This calculates the SIMD-accelerated Hamming Distance
    /// 
    /// <https://docs.rs/bio/latest/bio/alignment/distance/simd/fn.hamming.html>
    pub fn hamming_smid(alpha_str: String, beta_str: String) -> u64 {
        return bio::alignment::distance::simd::hamming(alpha_str.as_bytes(), beta_str.as_bytes());
    }
    /// This is a function for the Backward oracle matching algorithm
    /// 
    /// <https://docs.rs/bio/latest/bio/pattern_matching/bom/index.html>
    pub fn bom_matching(pattern: String, text: String) -> Vec<usize> {
        let bom = bio::pattern_matching::bom::BOM::new(pattern.as_bytes());
        return bom.find_all(text.as_bytes()).collect();
    }
    /// This is a function which calculates the Backward nondeterministic matching
    /// 
    /// <https://docs.rs/bio/latest/bio/pattern_matching/bndm/index.html>
    pub fn bndm_matching(pattern: String, text: String) -> Vec<usize> {
        let bndm = bio::pattern_matching::bndm::BNDM::new(pattern.as_bytes());
        return bndm.find_all(text.as_bytes()).collect();
    }
    /// This function which calculates the horspool matching
    /// 
    /// <https://docs.rs/bio/latest/bio/pattern_matching/horspool/index.html>
    pub fn horspool_matching(pattern: String, text: String) -> Vec<usize> {
        let horspool = bio::pattern_matching::horspool::Horspool::new(pattern.as_bytes());
        return horspool.find_all(text.as_bytes()).collect();
    }
    /// This function which calculates the kmp matching
    /// 
    /// <https://docs.rs/bio/latest/bio/pattern_matching/kmp/index.html>
    pub fn kmp_matching(pattern: String, text: String) -> Vec<usize> {
        let kmp = bio::pattern_matching::kmp::KMP::new(pattern.as_bytes());
        return kmp.find_all(text.as_bytes()).collect();
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test_kmp_matching_one() {
        assert_eq!(
            super::bio_algorithms::kmp_matching("aaa".to_string(), "bbbaaabbbaaabbb".to_string()),
            [3, 9]
        );
    }

    #[test]
    fn test_kmp_matching_two() {
        assert_eq!(
            super::bio_algorithms::kmp_matching(
                "aaa".to_string(),
                "bbbccccccbbbcccccccbbb".to_string()
            ),
            [] as [usize; 0]
        );
    }

    #[test]
    fn test_horspool_matching_one() {
        assert_eq!(
            super::bio_algorithms::horspool_matching(
                "aaa".to_string(),
                "bbbaaabbbaaabbb".to_string()
            ),
            [3, 9]
        );
    }
    #[test]
    fn test_horspool_matching_two() {
        assert_eq!(
            super::bio_algorithms::horspool_matching(
                "aaa".to_string(),
                "bbbccccccbbbcccccccbbb".to_string()
            ),
            [] as [usize; 0]
        );
    }

    #[test]
    fn test_bndm_matching_one() {
        assert_eq!(
            super::bio_algorithms::bndm_matching("aaa".to_string(), "bbbaaabbbaaabbb".to_string()),
            [3, 9]
        );
    }

    #[test]
    fn test_bndm_matching_two() {
        assert_eq!(
            super::bio_algorithms::bndm_matching(
                "aaa".to_string(),
                "bbbccccccbbbcccccccbbb".to_string()
            ),
            [] as [usize; 0]
        );
    }

    #[test]
    fn test_bom_matching_one() {
        assert_eq!(
            super::bio_algorithms::bom_matching("aaa".to_string(), "bbbaaabbbaaabbb".to_string()),
            [3, 9]
        );
    }
    #[test]
    fn test_bom_matching_two() {
        assert_eq!(
            super::bio_algorithms::bom_matching(
                "aaa".to_string(),
                "bbbccccccbbbcccccccbbb".to_string()
            ),
            [] as [usize; 0]
        );
    }

    #[test]
    fn test_levenshtein_smid_distance_one() {
        assert_eq!(
            super::bio_algorithms::levenshtein_smid("00110".to_string(), "00100".to_string()),
            1
        );
    }
    #[test]
    fn test_levenshtein_smid_distance_two() {
        assert_eq!(
            super::bio_algorithms::levenshtein_smid("10110".to_string(), "00100".to_string()),
            2
        );
    }
    #[test]
    fn test_levenshtein_smid_distance_three() {
        assert_eq!(
            super::bio_algorithms::levenshtein_smid("11110".to_string(), "00100".to_string()),
            3
        );
    }

    #[test]
    fn test_levenshtein_smid_distance_four() {
        assert_eq!(
            super::bio_algorithms::levenshtein_smid("11110".to_string(), "00101".to_string()),
            4
        );
    }

    #[test]
    fn test_hamming_house_baum() {
        assert_eq!(
            super::bio_algorithms::hamming("Baum".to_string(), "Haus".to_string()),
            2
        );
    }

    #[test]
    fn test_hamming_distance_one() {
        assert_eq!(
            super::bio_algorithms::hamming("00110".to_string(), "00100".to_string()),
            1
        );
    }

    #[test]
    fn test_hamming_distance_two() {
        assert_eq!(
            super::bio_algorithms::hamming("12345".to_string(), "13344".to_string()),
            2
        );
    }

    #[test]
    fn test_hamming_distance_three() {
        assert_eq!(
            super::bio_algorithms::hamming("44555".to_string(), "44666".to_string()),
            3
        );
    }

    #[test]
    fn test_hamming_distance_four() {
        assert_eq!(
            super::bio_algorithms::hamming("43555".to_string(), "44666".to_string()),
            4
        );
    }
    #[test]
    fn test_levenshtein_house_baum() {
        assert_eq!(
            super::bio_algorithms::levenshtein("Baum".to_string(), "Haus".to_string()),
            2
        );
    }
    #[test]
    fn test_levenshtein_rust_cpp() {
        assert_eq!(
            super::bio_algorithms::levenshtein("rust".to_string(), "cpp".to_string()),
            4
        );
    }

    #[test]
    fn test_levenshtein_distance_one() {
        assert_eq!(
            super::bio_algorithms::levenshtein("00110".to_string(), "00100".to_string()),
            1
        );
    }

    #[test]
    fn test_levenshtein_distance_two() {
        assert_eq!(
            super::bio_algorithms::levenshtein("12345".to_string(), "13344".to_string()),
            2
        );
    }

    #[test]
    fn test_levenshtein_distance_three() {
        assert_eq!(
            super::bio_algorithms::levenshtein("44555".to_string(), "44666".to_string()),
            3
        );
    }

    #[test]
    fn test_levenshtein_distance_four() {
        assert_eq!(
            super::bio_algorithms::levenshtein("43555".to_string(), "44666".to_string()),
            4
        );
    }
}
