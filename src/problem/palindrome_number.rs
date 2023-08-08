pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return x.to_string().chars().rev().eq(x.to_string().chars());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [(true, 121), (false, -121), (false, 10)];
        for (ans, x) in cases {
            assert_eq!(ans, Solution::is_palindrome(x));
        }
    }
}
