use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut map_s_to_t: HashMap<char, char> = HashMap::new();
    let mut map_t_to_s: HashMap<char, char> = HashMap::new();

    for (c1, c2) in s.chars().zip(t.chars()) {
        if let Some(&mapped_c1) = map_s_to_t.get(&c1) {
            if mapped_c1 != c2 {
                return false;
            }
        } else {
            map_s_to_t.insert(c1, c2);
        }

        if let Some(&mapped_c2) = map_t_to_s.get(&c2) {
            if mapped_c2 != c1 {
                return false;
            }
        } else {
            map_t_to_s.insert(c2, c1);
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_valid() {
        let out = is_isomorphic("egg".to_string(), "add".to_string());
        assert_eq!(out, true);
    }

    #[test]
    fn basic_invalid() {
        let out = is_isomorphic("foo".to_string(), "bar".to_string());
        assert_eq!(out, false);
    }

    #[test]
    fn basic_valid_2() {
        let out = is_isomorphic("paper".to_string(), "title".to_string());
        assert_eq!(out, true);
    }
    #[test]
    fn aaaah() {
        let out = is_isomorphic("bbbaaaba".to_string(), "aaabbbba".to_string());
        assert_eq!(out, false);
    }
}
