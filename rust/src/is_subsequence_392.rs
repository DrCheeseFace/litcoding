pub fn is_subsequence(s: String, t: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    let mut i = 0;
    for j in 0..t.len() {
        if i == s.len() {
            break;
        }
        if s[i] == t[j] {
            i += 1;
        }
    }
    i == s.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_valid() {
        let out = is_subsequence("abc".to_string(), "ahbgbc".to_string());
        assert_eq!(out, true);
    }
    #[test]
    fn basic_invalid() {
        let out = is_subsequence("abz".to_string(), "ahbgbc".to_string());
        assert_eq!(out, false);
    }
    #[test]
    fn whole_string() {
        let out = is_subsequence("abc".to_string(), "abc".to_string());
        assert_eq!(out, true);
    }
    #[test]
    fn subsequence_is_larger() {
        let out = is_subsequence("abc".to_string(), "a".to_string());
        assert_eq!(out, false);
    }
    #[test]
    fn single_letter_sub() {
        let out = is_subsequence("a".to_string(), "hbagbc".to_string());
        assert_eq!(out, true);
    }
}
