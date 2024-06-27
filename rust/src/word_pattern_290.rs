use std::collections::HashMap;

pub fn word_pattern(pattern: String, s: String) -> bool {
    if pattern.len() != s.split_whitespace().count() {
        return false;
    }
    let mut pattern_to_s: HashMap<char, &str> = HashMap::new();
    let mut s_to_pattern: HashMap<&str, char> = HashMap::new();
    for (p, st) in pattern.chars().zip(s.split_whitespace()) {
        if let Some(&mapped_p) = pattern_to_s.get(&p) {
            if mapped_p != st {
                return false;
            }
        } else {
            pattern_to_s.insert(p, st);
        }
        if let Some(&mapped_s) = s_to_pattern.get(st) {
            if mapped_s != p {
                return false;
            }
        } else {
            s_to_pattern.insert(st, p);
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_valid() {
        let out = word_pattern("abba".to_string(), "dog cat cat dog".to_string());
        assert_eq!(out, true)
    }

    #[test]
    fn basic_invalid() {
        let out = word_pattern("abba".to_string(), "dog cat cat fish".to_string());
        assert_eq!(out, false)
    }

    #[test]
    fn basic_invalid_2() {
        let out = word_pattern("aaaa".to_string(), "dog cat cat dog".to_string());
        assert_eq!(out, false)
    }

    #[test]
    fn repeating_invalid() {
        let out = word_pattern("abba".to_string(), "dog dog dog dog".to_string());
        assert_eq!(out, false)
    }

    #[test]
    fn repeating_s() {
        let out = word_pattern("abba".to_string(), "dog dog dog dog".to_string());
        assert_eq!(out, false)
    }
}
