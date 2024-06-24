pub fn is_palindrome(x: i32) -> bool {
    return x.to_string().chars().rev().eq(x.to_string().chars());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_digit() {
        assert_eq!(is_palindrome(1), true)
    }
    #[test]
    fn two_digits_correct() {
        assert_eq!(is_palindrome(22), true)
    }
    #[test]
    fn two_digits_incorrect() {
        assert_eq!(is_palindrome(12), false)
    }
    #[test]
    fn three_digits_incorrect() {
        assert_eq!(is_palindrome(123), false)
    }
    #[test]
    fn three_digits_correct() {
        assert_eq!(is_palindrome(121), true)
    }
}
