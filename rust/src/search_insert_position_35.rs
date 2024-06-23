pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_insert_position_1() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn search_insert_position_2() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn search_insert_position_3() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }

    #[test]
    fn search_insert_position_4() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
    }

    #[test]
    fn search_insert_position_5() {
        assert_eq!(search_insert(vec![1], 0), 0);
    }
}
