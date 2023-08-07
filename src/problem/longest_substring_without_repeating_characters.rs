pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut ans = 0;
        let mut left = 0;
        let mut asciis = vec![false; 128];

        for (right, &byte) in bytes.into_iter().enumerate() {
            let idx = byte as usize;
            while asciis[idx] {
                asciis[bytes[left] as usize] = false;
                left += 1;
            }
            asciis[idx] = true;
            ans = ans.max(right - left + 1);
        }
        return ans as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [(3, "abcabcbb"), (1, "bbbbb"), (3, "pwwkew")];
        for (ans, s) in cases {
            assert_eq!(ans, Solution::length_of_longest_substring(s.to_string()));
        }
    }
}
