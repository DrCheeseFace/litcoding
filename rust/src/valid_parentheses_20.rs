pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in s.chars() {
        match char {
            '(' | '[' | '{' => stack.push(char),
            ')' => match stack.last() {
                Some(lastchar) => {
                    if lastchar.ne(&'(') {
                        return false;
                    }
                    stack.pop();
                }
                None => return false,
            },
            ']' => match stack.last() {
                Some(lastchar) => {
                    if lastchar.ne(&'[') {
                        return false;
                    }
                    stack.pop();
                }
                None => return false,
            },
            '}' => match stack.last() {
                Some(lastchar) => {
                    if lastchar.ne(&'{') {
                        return false;
                    }
                    stack.pop();
                }
                None => return false,
            },
            _ => return false,
        }
    }
    if stack.is_empty() {
        return true;
    }
    return false;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valid_parentheses_1() {
        assert_eq!(is_valid("()".to_string()), true);
    }
    #[test]
    fn valid_parentheses_2() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }
    #[test]
    fn valid_parentheses_3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
    #[test]
    fn valid_parentheses_4() {
        assert_eq!(is_valid("([)]".to_string()), false);
    }
    #[test]
    fn valid_parentheses_5() {
        assert_eq!(is_valid("{[]}".to_string()), true);
    }
    #[test]
    fn valid_parentheses_6() {
        assert_eq!(is_valid("".to_string()), true);
    }
    #[test]
    fn valid_parentheses_7() {
        assert_eq!(is_valid("{".to_string()), false);
    }
    #[test]
    fn valid_parentheses_8() {
        assert_eq!(is_valid("]".to_string()), false);
    }
}
