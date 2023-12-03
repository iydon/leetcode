use crate::util::io::Input;

struct Problem;

impl Problem {
    pub fn new() -> Self {
        return Problem {};
    }

    pub fn solve(&self, a: f64, b: f64) -> f64 {
        let ratio = f64::ln(b / a) / f64::ln(3.0 / 2.0);
        return (ratio + 1.0).floor();
    }

    pub fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        let ans = self.solve(input.scalar(), input.scalar());
        writeln!(stdout, "{ans}").unwrap();
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
        assert_eq!(Problem::_test("4 7\n"), "2\n");
        assert_eq!(Problem::_test("4 9\n"), "3\n");
        assert_eq!(Problem::_test("1 1\n"), "1\n");
    }
}

pub fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
