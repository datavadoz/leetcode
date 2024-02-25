/*******************************************************************************
 * [LEET-00001] Two Sum
 * Level: Easy
 * Describtion: Given an array of integers nums and an integer target, return
 *      indices of the two numbers such that they add up to target. You may
 *      assume that each input would have exactly one solution, and you may not
 *      use the same element twice. You can return the answer in any order.
 ******************************************************************************/
use std::collections::HashMap;

struct Solution {}

impl Solution {
    /// Time: n^2 | Space: 1
    fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_len = nums.len();
        for i in 0..nums_len-1 {
            for j in i+1..nums_len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    // Time: n | Space: n
    fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_len = nums.len();
        let mut my_hm: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums_len {
            let subtract = target - &nums[i];
            if let Some(&j) = my_hm.get(&subtract) {
                return vec![j, i as i32];
            }
            my_hm.insert(nums[i], i as i32);
        }
        vec![]
    }
}

fn main() {
    let result = Solution::two_sum_1(vec![2, 7, 11, 15], 9);
    println!("Result: {:?}", result);

    let result = Solution::two_sum_2(vec![2, 7, 11, 15], 9);
    println!("Result: {:?}", result);
}
