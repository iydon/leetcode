use crate::util::io::Input;

struct Problem;

impl Problem {
    pub fn new() -> Self {
        return Problem {};
    }

    pub fn solve(&self, ith: i8, jth: i8) -> i8 {
        return i8::abs(ith as i8 - 2) + i8::abs(jth as i8 - 2);
    }

    pub fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        for ith in 0..5 {
            if let Some(jth) = input.text().split_whitespace().position(|s| s == "1") {
                writeln!(stdout, "{}", self.solve(ith as i8, jth as i8)).unwrap();
            }
        }
    }

    fn _test(input: &str) -> String {
        let mut output = Vec::new();
        let problem = Self::new();
        problem.via_io(input.as_bytes(), &mut output);
        return String::from_utf8(output).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Problem::_test("0 0 0 0 0\n0 0 0 0 1\n0 0 0 0 0\n0 0 0 0 0\n0 0 0 0 0\n"), "3\n");
        assert_eq!(Problem::_test("0 0 0 0 0\n0 0 0 0 0\n0 1 0 0 0\n0 0 0 0 0\n0 0 0 0 0\n"), "1\n");
    }
}

pub fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
