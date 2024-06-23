pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut sorted_strs = strs.clone();
    let _ = sorted_strs.sort_by(|a, b| a.len().cmp(&b.len())).clone();
    let mut prefix: String = "".to_string();
    for (index, i) in sorted_strs.get(0).unwrap().chars().enumerate() {
        for j in sorted_strs.iter().skip(0) {
            if j.chars().nth(index).unwrap() != i {
                return prefix;
            }
        }
        prefix.push_str(&i.to_string());
    }

    return prefix;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_char() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string(),
            ]),
            "fl".to_string()
        );
    }
    #[test]
    fn no_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string(),
            ]),
            "".to_string()
        );
    }
}
