pub fn is_palindrome(s: String) -> bool {
    // let filtered: String = s
    //     .to_ascii_lowercase()
    //     .chars()
    //     .filter(|c| c.is_alphanumeric())
    //     .collect();
    //
    // if filtered.len() == 0 {
    //     return true;
    // }
    // let string_size = filtered.len() - 1;
    // for i in 0..=string_size / 2 {
    //     if filtered.chars().nth(i).unwrap() != filtered.chars().nth(string_size - i).unwrap() {
    //         return false;
    //     }
    // }
    // return true;

    let s: Vec<_> = s
        .chars()
        .filter_map(|c| c.is_ascii_alphanumeric().then(|| c.to_ascii_lowercase()))
        .collect();

    s.iter().eq(s.iter().rev())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn is_palendrome_with_formatting_needed() {
        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }
    #[test]
    fn invalid_palidrome() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }
    #[test]
    fn empty_string() {
        assert_eq!(is_palindrome(" ".to_string()), true);
    }
    #[test]
    fn short_test() {
        assert_eq!(is_palindrome("aabaa".to_string()), true);
    }
}
