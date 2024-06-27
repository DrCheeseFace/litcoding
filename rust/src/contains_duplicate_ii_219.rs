use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for (index, i) in nums.iter().enumerate() {
        if let Some(&mapped) = h.get(i) {
            if (index as i32 - &mapped).abs() <= k {
                return true;
            }
        }
        h.insert(*i, index as i32);
    }
    return false;
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn basic_valid() {
        let out = contains_nearby_duplicate(vec![1, 2, 3, 1], 3);
        assert_eq!(out, true);
    }

    #[test]
    fn ghuh() {
        let out = contains_nearby_duplicate(vec![1, 0, 1, 1], 1);
        assert_eq!(out, true);
    }

    #[test]
    fn ais() {
        let out = contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2);
        assert_eq!(out, false);
    }
}
