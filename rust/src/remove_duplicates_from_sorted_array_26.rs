pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    return nums.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}
