use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::<i32, i32>::new();
        let size = nums.len();
        for i in 0..size {
            if mp.contains_key(&(target - nums[i])) {
                return vec![mp[&(target - nums[i])], i as i32];
            }
            mp.insert(nums[i], i as i32);
        }
        return vec![];
    }
}
