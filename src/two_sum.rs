// https://leetcode.com/problems/two-sum/description/
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // FIRST SOLUTION - brute force
    // let len = nums.len();
    // let mut result = Vec::new();
    //
    // for (left_index, left_num) in  nums.iter().enumerate() {
    //     let next_index = left_index + 1;
    //     if next_index < len {
    //         for right_index in (left_index+1)..len {
    //             let right_num = nums.get(right_index).unwrap();
    //             if *left_num + *right_num == target {
    //                 result.push(left_index as i32);
    //                 result.push(right_index as i32);
    //                 return result;
    //             }
    //         }
    //     }
    // }
    //
    // result

    // SECOND SOLUTION - Hash map
    let mut hash_map = HashMap::with_capacity(nums.len());
    for (i, &number) in nums.iter().enumerate() {
        match hash_map.get(&number) {
            Some(&j) => {
                return vec![j as i32, i as i32];
            }
            None => {
                hash_map.insert(target - number, i);
            }
        }
    }

    vec![0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0,1]);
    }

    #[test]
    fn example_two() {
        let nums = vec![3,2,4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn example_three() {
        let nums = vec![3,3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0,1]);
    }
}


