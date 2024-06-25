pub fn add_binary(a: String, b: String) -> String {
    let len: usize = a.len().max(b.len()) + 1;
    let mut result: String = String::new();
    let a: String = a
        .chars()
        .rev()
        .chain(std::iter::repeat('0'))
        .take(len)
        .collect();
    let b: String = b
        .chars()
        .rev()
        .chain(std::iter::repeat('0'))
        .take(len)
        .collect();

    let mut carry: char = '0';
    for (index, bit) in a.chars().enumerate() {
        if bit == b.chars().nth(index).unwrap() {
            if bit == '1' {
                if carry == '1' {
                    result.push('1');
                } else {
                    result.push('0');
                    carry = '1'
                }
            } else {
                if carry == '1' {
                    result.push('1');
                    carry = '0'
                } else {
                    result.push('0');
                }
            }
        } else {
            if carry == '1' {
                result.push('0');
                carry = '1'
            } else {
                result.push('1');
                carry = '0'
            }
        }
    }
    if carry == '1' {
        result.push('1');
    }
    if result.chars().last().unwrap() == '0' {
        result.pop();
    }
    return result.chars().rev().collect();
}

#[cfg(test)]
mod test {
    use super::add_binary;

    #[test]
    fn same_len() {
        assert_eq!(
            add_binary("1011".to_string(), "0111".to_string()),
            "10010".to_string()
        );
    }
    #[test]
    fn first_shorter() {
        assert_eq!(
            add_binary("1011".to_string(), "10111".to_string()),
            "100010".to_string()
        );
    }
    #[test]
    fn second_shorter() {
        assert_eq!(
            add_binary("11011".to_string(), "1011".to_string()),
            "100110".to_string()
        );
    }
    #[test]
    fn other() {
        assert_eq!(
            add_binary("0011".to_string(), "1000".to_string()),
            "1011".to_string()
        );
    }
    #[test]
    fn carry() {
        assert_eq!(
            add_binary("111".to_string(), "111".to_string()),
            "1110".to_string()
        );
    }
}
