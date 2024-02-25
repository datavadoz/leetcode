/*******************************************************************************
 * [LEET-00001] Two Sum
 * Level: Easy
 * Describtion: Given an array of integers nums and an integer target, return
 *      indices of the two numbers such that they add up to target. You may
 *      assume that each input would have exactly one solution, and you may not
 *      use the same element twice. You can return the answer in any order.
 ******************************************************************************/
use leetcode::leet_00001;

fn main() {
    leet_00001::Solution::two_sum_1(vec![2, 7, 11, 15], 9);
    leet_00001::Solution::two_sum_2(vec![2, 7, 11, 15], 9);
}
