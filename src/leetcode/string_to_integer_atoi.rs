enum State {
    Start,
    Symbol,
    Number,
    End,
}

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut state = State::Start;
        let mut is_neg = false;
        let mut answer = 0i64;
        let maximal = (i32::MAX as i64) + 1;
        for byte in s.bytes() {
            match state {
                State::Start => match byte {
                    b' ' => continue,
                    b'0'..=b'9' => {
                        answer = answer * 10 + (byte - b'0') as i64;
                        state = State::Number;
                    }
                    b'-' | b'+' => {
                        is_neg = byte == b'-';
                        state = State::Symbol;
                    }
                    _ => {
                        state = State::End;
                    }
                },
                State::Symbol | State::Number => match byte {
                    b'0'..=b'9' => {
                        answer = answer * 10 + (byte - b'0') as i64;
                        state = State::Number;
                        if answer >= maximal {
                            state = State::End;
                        }
                    }
                    _ => {
                        state = State::End;
                    }
                },
                State::End => break,
            }
        }

        return if answer >= maximal {
            if is_neg {
                i32::MIN
            } else {
                i32::MAX
            }
        } else {
            if is_neg {
                -answer as i32
            } else {
                answer as i32
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [(42, "42"), (-42, "-42"), (4193, "4193 with words")];
        for (ans, s) in cases {
            assert_eq!(ans, Solution::my_atoi(s.to_string()));
        }
    }
}
