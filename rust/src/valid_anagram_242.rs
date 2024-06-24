use std::collections::HashMap;

// pub fn is_anagram(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }
//
//     let mut h: HashMap<char, i32> = HashMap::new();
//     for i in s.chars() {
//         if let Some(value) = h.get_mut(&i) {
//             *value = *value + 1;
//         } else {
//             h.insert(i, 1);
//         }
//     }
//
//     for j in t.chars() {
//         if let Some(value) = h.get_mut(&j) {
//             *value = *value - 1;
//             if *value == 0 {
//                 h.remove_entry(&j);
//             }
//         } else {
//             return false;
//         }
//     }
//     return true;
// }

pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = std::collections::HashMap::new();

    for letter in s.chars() {
        *map.entry(letter).or_insert(0) += 1;
    }
    for letter in t.chars() {
        *map.entry(letter).or_insert(0) += -1;
    }
    for (key, value) in map {
        if (value != 0) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }
    #[test]
    fn shouldbefalse() {
        assert_eq!(is_anagram("rat".to_string(), "car".to_string()), false);
    }
}
