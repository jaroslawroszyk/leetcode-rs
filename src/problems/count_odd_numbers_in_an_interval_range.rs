pub struct Solution {}

impl Solution {
    fn is_odd(n: i32) -> bool {
        n % 2 == 1
    }

    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut odd = (high - low) / 2;
        if Solution::is_odd(low) || Solution::is_odd(high) {
            odd += 1;
        }
        odd
    }
}

pub fn count_odd_numbers_in_an_interval_range_testcase() {
    let res = Solution::count_odds(1, 10);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use crate::problems::count_odd_numbers_in_an_interval_range::*;

    #[test]
    fn should_return_five_number_in_range_from_one_to_ten() {
        let result = Solution::count_odds(1, 10);
        let expected = 5;
        assert_eq!(result, expected);
    }
}
