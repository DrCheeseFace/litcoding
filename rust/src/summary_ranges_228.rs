pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return [].to_vec();
    }
    if nums.len() == 1 {
        return vec![nums[0].to_string()];
    }

    let mut out: Vec<String> = Vec::new();
    let mut count: i32 = 0;

    for index in 0..nums.len() - 1 {
        if nums[index + 1] - 1 != nums[index] {
            if count == 0 {
                out.push(format!("{:?}", nums[index]));
            } else {
                out.push(format!(
                    "{:?}->{:?}",
                    nums[index - count as usize],
                    nums[index]
                ));
            }
            count = 0;
        } else {
            count += 1;
        }
        if index + 1 == nums.len() - 1 {
            println!("{:?}", index);
            if count == 0 {
                out.push(format!("{:?}", nums[index + 1]));
            } else {
                out.push(format!(
                    "{:?}->{:?}",
                    nums[index - count as usize],
                    nums[index + 1]
                ));
            }
        }
    }

    return out;
}

// Output: ["0->2","4->5","7"]
// Output: ["0","2->4","6","8->9"]

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let out = summary_ranges(vec![0, 1, 2, 4, 5, 7]);
        assert_eq!(out, vec!["0->2", "4->5", "7"])
    }
    #[test]
    fn test_2() {
        let out = summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]);
        assert_eq!(out, vec!["0", "2->4", "6", "8->9"])
    }

    #[test]
    fn full_len() {
        let out = summary_ranges(vec![1, 2, 3]);
        assert_eq!(out, vec!["0->2"])
    }
}
