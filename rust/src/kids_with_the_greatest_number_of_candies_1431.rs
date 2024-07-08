pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max: i32 = *candies.iter().max().unwrap();
    let mut out: Vec<bool> = Vec::new();
    for i in candies {
        if i + extra_candies >= max {
            out.push(true);
        } else {
            out.push(false);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kids_with_candies() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let result = kids_with_candies(candies, extra_candies);
        assert_eq!(result, vec![true, true, true, false, true]);
    }
}
