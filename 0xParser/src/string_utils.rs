// https://users.rust-lang.org/t/how-to-get-a-substring-of-a-string/1351/11
use std::ops::{Bound, RangeBounds};

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> String;
    fn slice(&self, range: impl RangeBounds<usize>) -> String;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> String {
        self.chars().skip(start).take(len).collect::<String>()
    }

    fn slice(&self, range: impl RangeBounds<usize>) -> String {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

#[cfg(test)]
mod string_utils_tests {
    use crate::string_utils::StringUtils;

    #[test]
    fn test_substring() {
        let test_string = String::from("Hello There!");
        assert_eq!(test_string.substring(0, 100), "Hello There!");
        assert_eq!(test_string.substring(0, 2), "He");
        assert_eq!(test_string.substring(1, 4), "ello");

        let test_string = String::from("");
        assert_eq!(test_string.substring(0, 100), "");
        assert_eq!(test_string.substring(0, 2), "");
        assert_eq!(test_string.substring(1, 4), "");
    }
}