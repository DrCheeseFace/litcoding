use std::collections::HashMap;

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut h: HashMap<i32, u32> = HashMap::new();
    let max: usize = nums.len() / 2;

    if max == 0 {
        return nums[0];
    }

    for i in nums.iter() {
        if let Some(value) = h.get_mut(i) {
            if *value + 1 > max as u32 {
                return *i;
            }
            *value = *value + 1;
        } else {
            h.insert(*i, 1);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
        assert_eq!(majority_element(vec![1]), 1);
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
