pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        return Self::longest_window(s);
    }

    #[inline]
    pub fn longest_dynamic(s: String) -> String {
        let size = s.len();
        let chars: Vec<char> = s.chars().collect();

        let mut dp = vec![vec![true; size]; size];
        let (mut start, mut end) = (0, 0);

        for kth in 1..size {
            for ith in 0..(size - kth) {
                if kth == 1 {
                    dp[ith][ith + kth] = chars[ith] == chars[ith + 1];
                } else {
                    dp[ith][ith + kth] =
                        (chars[ith] == chars[ith + kth]) && dp[ith + 1][ith + kth - 1];
                }

                if dp[ith][ith + kth] {
                    (start, end) = (ith, ith + kth);
                }
            }
        }

        return chars[start..=end].into_iter().collect::<String>();
    }

    #[inline]
    pub fn longest_window(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let (mut head, mut window) = (0, s.len());
        while head != chars.len() {
            if head + window > chars.len() {
                window -= 1;
                head = 0;
                continue;
            }
            let temp = &chars[head..head + window];
            if Self::_longest_window_is_palindrome(temp) {
                return temp.iter().collect();
            }
            head += 1;
        }
        return "".to_string();
    }

    fn _longest_window_is_palindrome(chars: &[char]) -> bool {
        let size = chars.len();
        for ith in 0..size / 2 {
            if chars[ith] != chars[size - ith - 1] {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [("bab", "babcd"), ("bb", "cbbd")];
        for (ans, s) in cases {
            assert_eq!(ans, Solution::longest_palindrome(s.to_string()));
        }
    }
}
