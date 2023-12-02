use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices = HashMap::new();
        for (ith, num) in nums.into_iter().enumerate() {
            let jth: i32 = ith as i32;
            match indices.get(&(target - num)) {
                Some(&kth) => return vec![kth, jth],
                None => indices.insert(num, jth),
            };
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [
            (vec![0, 1], vec![2, 7, 11, 15], 9),
            (vec![1, 2], vec![3, 2, 4], 6),
            (vec![0, 1], vec![3, 3], 6),
        ];
        for (ans, nums, target) in cases {
            assert_eq!(ans, Solution::two_sum(nums, target));
        }
    }
}
