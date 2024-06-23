use std::collections::HashMap;
pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut magazine_map: HashMap<char, i32> = HashMap::new();
    for c in magazine.chars() {
        let count = magazine_map.entry(c).or_insert(0);
        *count += 1;
    }

    for c in ransom_note.chars() {
        let count = magazine_map.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }

    return true;

    //
    //
    //
    //
    //
    //
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_test_1() {
        assert_eq!(can_construct("a".to_string(), "b".to_string()), false);
    }
    #[test]
    fn can_construct_test_2() {
        assert_eq!(can_construct("aa".to_string(), "ab".to_string()), false);
    }
    #[test]
    fn can_construct_test_3() {
        assert_eq!(can_construct("aa".to_string(), "aab".to_string()), true);
    }
}
