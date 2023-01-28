use std::collections::HashMap;

struct Solution1;
struct Solution2;

impl Solution1 {
    /// Time complexity: O(n^2)
    /// Space complexity: O(1)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_len = nums.len();
        for i in 0..(nums_len - 1) {
            for j in (i + 1)..nums_len {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![];
    }
}

impl Solution2 {
    /// Time complexity: O(n)
    /// Space complexity: O(n)
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_dict = HashMap::<i32, i32>::new();
        for i in 0..nums.len() {
            let tmp = target - nums[i];
            if let Some(x) = my_dict.get(&tmp) {
                return vec![*x, i as i32];
            }
            my_dict.insert(nums[i], i as i32);
        }
        return vec![];
    }
}


fn main() {
    let nums: Vec<i32> = vec![3, 2, 4];
    let target: i32 = 6; 
    let solution = 2;
    let ans: Vec<i32>;

    if solution == 1 {
        ans = Solution1::two_sum(nums, target);
    } else if solution == 2 {
        ans = Solution2::two_sum(nums, target)
    } else {
        unimplemented!();
    }
    println!("{:?}", ans);
}

