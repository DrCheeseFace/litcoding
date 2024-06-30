pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut out: String = String::new();
    for (i, j) in word1.chars().zip(word2.chars()) {
        out.push(i);
        out.push(j);
    }

    if word1.len() > word2.len() {
        out.push_str((&word1[word2.len()..]));
    } else {
        out.push_str((&word2[word1.len()..]));
    }

    return out;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn merge_strings_alternately_1() {
        assert_eq!(
            merge_alternately("abc".to_string(), "pqr".to_string()),
            "apbqcr".to_string()
        );
    }
    #[test]
    fn merge_strings_alternately_2() {
        assert_eq!(
            merge_alternately("ab".to_string(), "pqrs".to_string()),
            "apbqrs".to_string()
        );
    }
    #[test]
    fn merge_strings_alternately_3() {
        assert_eq!(
            merge_alternately("abcd".to_string(), "pq".to_string()),
            "apbqcd".to_string()
        );
    }
}
