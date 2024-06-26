pub fn str_str(haystack: String, needle: String) -> i32 {
    // if haystack.len() < needle.len() {
    //     return -1
    // }
    // let mut larger_pointer: usize = 0;
    // let mut smaller_pointer: usize = 0;
    // let mut count: usize = 0;
    // while larger_pointer <= haystack.len() - needle.len() {
    //     if haystack
    //         .chars()
    //         .nth(count)
    //         .eq(&needle.chars().nth(smaller_pointer))
    //     {
    //         smaller_pointer += 1;
    //         count += 1;
    //         if smaller_pointer == needle.len() {
    //             return larger_pointer as i32;
    //         }
    //     } else {
    //         smaller_pointer = 0;
    //         larger_pointer += 1;
    //         count = larger_pointer;
    //     }
    // }
    // return -1;
    haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_valid() {
        let out = str_str("sadbutsad".to_string(), "sad".to_string());
        assert_eq!(out, 0)
    }

    #[test]
    fn basic_invalid() {
        let out = str_str("leetcode".to_string(), "leeto".to_string());
        assert_eq!(out, -1)
    }

    #[test]
    fn single_letter_only() {
        let out = str_str("i".to_string(), "i".to_string());
        assert_eq!(out, 0)
    }

    #[test]
    fn single_letter_inside_long() {
        let out = str_str("aksalslasld".to_string(), "l".to_string());
        assert_eq!(out, 4)
    }

    #[test]
    fn middle_of_string() {
        let out = str_str("mynameistharun".to_string(), "name".to_string());
        assert_eq!(out, 2)
    }

    #[test]
    fn end_of_string() {
        let out = str_str("mynameischarlie".to_string(), "charlie".to_string());
        assert_eq!(out, 8)
    }
    #[test]
    fn whole_string() {
        let out = str_str("poggers".to_string(), "poggers".to_string());
        assert_eq!(out, 0)
    }
    #[test]
    fn misisipi() {
        let out = str_str("mississippi".to_string(), "issip".to_string());
        assert_eq!(out, 4)
    }
    #[test]
    fn small_haystack() {
        let out = str_str("aaa".to_string(), "aaaaaaa".to_string());
        assert_eq!(out, -1)
    }
}
