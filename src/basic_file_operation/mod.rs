//! Collection of basic file operations
pub mod basic_file_operation {
    use std::fs;
    use std::io::Write;
    use std::io::{ self, BufRead };
    use std::path::Path;

    /// Read file and put it into a string
    pub fn read_file_to_string(file_path: impl AsRef<Path>) -> String {
        let res = fs::read_to_string(file_path);
        match res {
            Ok(res) => {
                return res;
            }
            Err(err) => panic!("{:?}", err),
        }
    }

    /// Write the content of a string vector to file
    pub fn write_file_from_char_vec(
        file_path: impl AsRef<Path>,
        input_vec: Vec<String>,
        append: bool
    ) -> bool {
        let file = fs::OpenOptions::new().write(true).create(true).append(append).open(file_path);
        let mut file = match file {
            Ok(res) => res,
            Err(_err) => {
                return false;
            }
        };
        for name in input_vec.iter() {
            writeln!(file, "{}", name).unwrap();
        }
        return true;
    }

    /// Read a file to String Vec
    pub fn read_file_to_str_vec(file_path: impl AsRef<Path>) -> Option<Vec<String>> {
        let file = fs::File::open(file_path);
        let mut vec = Vec::new();
        let file = match file {
            Ok(res) => res,
            Err(_err) => {
                return None;
            }
        };
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            vec.push(line.unwrap());
        }
        return Some(vec);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_write_file_from_char_vec_one_line() {
        let mut test_vec = Vec::new();
        for i in 1..10000 {
            test_vec.push(format!("String {}", i));
        }

        assert_eq!(
            super::basic_file_operation::write_file_from_char_vec(
                "tests/test_write_file/test_file_one.txt".to_string(),
                test_vec,
                false
            ),
            true
        );
    }
    #[test]
    fn test_read_file_to_vec() {
        let out_vec = super::basic_file_operation::read_file_to_str_vec(
            "tests/test_write_file/test_file_one.txt".to_string()
        );
        assert_eq!(out_vec.unwrap().len(), 9999);
    }

    #[test]
    fn test_read_file_to_string() {
        assert_eq!(
            super::basic_file_operation::read_file_to_string(
                "tests/test_read_file_to_string/sample_1.txt"
            ),
            "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."
        );
    }
}