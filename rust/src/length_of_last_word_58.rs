pub fn length_of_last_word(s: String) -> i32 {
    return s.split_whitespace().last().unwrap().len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    }
    #[test]
    fn hello() {
        assert_eq!(length_of_last_word("hello".to_string()), 5);
    }
    #[test]
    fn i() {
        assert_eq!(length_of_last_word("i".to_string()), 1);
    }
    #[test]
    fn this_1() {
        assert_eq!(length_of_last_word("this 1".to_string()), 1);
    }
    #[test]
    fn this1() {
        assert_eq!(length_of_last_word("this1".to_string()), 5);
    }
    #[test]
    fn this_1_hithere() {
        assert_eq!(length_of_last_word("this 1 hithere".to_string()), 7);
    }
    #[test]
    fn this_hippopotemus() {
        assert_eq!(length_of_last_word("this hippopotemus".to_string()), 12);
    }
}
