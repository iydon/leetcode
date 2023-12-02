pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ans = 0;

        while x != 0 {
            if ans > i32::MAX / 10 || ans < i32::MIN / 10 {
                return 0;
            }
            ans = ans * 10 + x % 10;
            x /= 10;
        }
        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [(321, 123), (-321, -123), (21, 120), (0, 0)];
        for (ans, x) in cases {
            assert_eq!(ans, Solution::reverse(x));
        }
    }
}
