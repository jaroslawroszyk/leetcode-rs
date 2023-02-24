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

pub fn two_sum_testcase() {
    // ms = [2,7,11,15], target = 9
    let input = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(input, target);
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::problems::two_sum::*;

    #[test]
    fn should_return_zero_and_one() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(input, target);
        let expected = vec![0,1];
        assert_eq!(result, expected);
    }
}
