pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut s1 = str1.clone();
    s1 += &str2.clone();

    let mut s2 = str2.clone();
    s2 += &str1.clone();

    if s1 != s2 {
        return String::from("");
    }

    str1.get(0..gcd(str1.len(), str2.len()))
        .unwrap()
        .to_string()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn abcabc() {
        let out = gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
        assert_eq!(out, "ABC".to_string());
    }

    #[test]
    fn ababab() {
        let out = gcd_of_strings("ABABAB".to_string(), "ABAB".to_string());
        assert_eq!(out, "AB".to_string());
    }

    #[test]
    fn leet_code() {
        let out = gcd_of_strings("LEET".to_string(), "CODE".to_string());
        assert_eq!(out, "".to_string());
    }
    #[test]
    fn taux() {
        let out = gcd_of_strings(
            "TAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
            "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX".to_string(),
        );
        assert_eq!(out, "TAUXX".to_string());
    }

    #[test]
    fn uneven() {
        let out = gcd_of_strings("ABCABCABC".to_string(), "ABCAAA".to_string());
        assert_eq!(out, "".to_string());
    }
}
