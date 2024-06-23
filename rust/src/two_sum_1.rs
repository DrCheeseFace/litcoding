use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if (i != j) && (nums.get(i).unwrap() + nums.get(j).unwrap() == target) {
                let this: Vec<i32> = vec![i.try_into().unwrap(), j.try_into().unwrap()];
                return this;
            }
        }
    }
    return nums;


    //     for (i, &num) in nums.iter().enumerate() {
    //         // Check if such a difference exists in the hash map
    //         match hm.get(&num) {
    //             // If it does, return the indices of the current number and the number with the difference
    //             Some(&j) => return vec![i as i32, j as i32],
    //             // If it doesn't, add the difference between target and the current number to the hash map
    //             None => {
    //                 hm.insert(target - num, i);
    //             }
    //         }
    //     }
    //     unreachable!();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashSet;
//
//     #[test]
//     fn test_two_sum1() {
//         assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
//         let results = two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]) ;
//     }
//
//     #[test]
//     fn test_two_sum2() {
//         assert_eq!(two_sum(vec![3, 2, 4], 6), vec![2, 1]);
//     }
//     #[test]
//     fn test_two_sum3() {
//         assert_eq!(two_sum(vec![3, 3], 6), vec![1, 0]);
//     }
// }
