use std::cmp::Ordering;
use std::collections::HashMap;

use crate::util::io::Input;

struct Problem;

impl Problem {
    pub fn new() -> Self {
        return Problem {};
    }

    pub fn solve(&self, line: String) -> Ordering {
        let mut counter = HashMap::with_capacity(2);
        for char in line.chars() {
            *counter.entry(char).or_insert(0) += 1;
        }
        let anton = counter.get(&'A').unwrap_or(&0);
        let danik = counter.get(&'D').unwrap_or(&0);
        return anton.cmp(danik);
    }

    pub fn via_io<I, O>(self, mut stdin: I, mut stdout: O)
    where
        I: std::io::BufRead,
        O: std::io::Write,
    {
        let mut input = Input::new(&mut stdin);
        input.skip();
        match self.solve(input.text()) {
            Ordering::Equal => writeln!(stdout, "Friendship").unwrap(),
            Ordering::Greater => writeln!(stdout, "Anton").unwrap(),
            Ordering::Less => writeln!(stdout, "Danik").unwrap(),
        };
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
        assert_eq!(Problem::_test("6\nADAAAA\n"), "Anton\n");
        assert_eq!(Problem::_test("7\nDDDAADA\n"), "Danik\n");
        assert_eq!(Problem::_test("6\nDADADA\n"), "Friendship\n");
    }
}

pub fn main() {
    let (stdin, stdout) = (std::io::stdin(), std::io::stdout());
    let output = std::io::BufWriter::new(stdout.lock());
    Problem::new().via_io(stdin.lock(), output);
}
