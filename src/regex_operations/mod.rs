//! Module for some regex operation
pub mod regex_operations {
    // This function checks if an input string matches with an regex input
    pub fn match_regex_string(regex_input: String, seach_string: String) -> bool {
        let regex = match regex::Regex::new(&regex_input) {
            Ok(res) => res,
            Err(err) => panic!("{:?}", err),
        };
        return regex.is_match(&seach_string);
    }
    // This function function replaces a regex in a input string
    pub fn replace_regex_string(
        input_string: String,
        regex_input: String,
        replace_str: String
    ) -> String {
        let regex = match regex::Regex::new(&regex_input) {
            Ok(res) => res,
            Err(err) => panic!("{:?}", err),
        };
        return String::from(regex.replace(&input_string, &replace_str));
    }

    // This function split a string with regex rules
    pub fn split_regex_string(input_string: String, regex_input: String) -> Vec<String> {
        let regex = match regex::Regex::new(&regex_input) {
            Ok(res) => res,
            Err(err) => panic!("{:?}", err),
        };
        let result: Vec<&str> = regex.split(&input_string).collect();
        let output: Vec<String> = result
            .iter()
            .map(|&s| { String::from(s) })
            .collect();
        return output;
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_regex_split_string_space() {
        assert_eq!(
            super::regex_operations::split_regex_string(
                String::from("a b c d e"),
                String::from("[ ]+")
            ),
            vec!["a", "b", "c", "d", "e"]
        );
    }

    #[test]
    fn test_regex_replace_12456() {
        assert_eq!(
            super::regex_operations::replace_regex_string(
                String::from("1078910"),
                String::from("[^01]+"),
                String::from("")
            ),
            "1010"
        );
    }
    #[test]
    fn test_extract_plz_from_string() {
        assert_eq!(
            super::regex_operations::replace_regex_string(
                String::from("1020 Wien"),
                String::from("[^0-9]+"),
                String::from("")
            ),
            "1020"
        );
    }

    #[test]
    fn test_regex_helloworld() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("^hello world"),
                String::from("hello world")
            ),
            true
        );
    }
    #[test]
    fn test_is_int() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("^[0-9]*$"),
                String::from("153020")
            ),
            true
        );
    }
    #[test]
    fn test_is_email() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("^[\\w\\.]+@([\\w]+\\.)+[\\w]{2,4}$"),
                String::from("test123@gmail.com")
            ),
            true
        );
    }

    #[test]
    fn test_float_number() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("\\-?\\d+[\\.\\d+]"),
                String::from("123.678")
            ),
            true
        );
    }

    #[test]
    fn test_not_float_number() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("\\-?\\d+[\\.\\d+]"),
                String::from("666.333")
            ),
            true
        );
    }

    #[test]
    fn test_end_with_rust() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("rust$"),
                String::from("I like rust")
            ),
            true
        );
    }

    #[test]
    fn test_start_with_rust() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("^rust"),
                String::from("rust is a nice language")
            ),
            true
        );
    }

    #[test]
    fn test_is_exactly_rust() {
        assert_eq!(
            super::regex_operations::match_regex_string(
                String::from("^rust$"),
                String::from("rust")
            ),
            true
        );
    }
}