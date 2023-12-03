use std::io::BufRead;
use std::str::FromStr;

pub struct Input<I: BufRead> {
    stdin: I,
    buffer: Vec<String>,
}

impl<I: BufRead> Input<I> {
    pub fn new(stdin: I) -> Self {
        return Self {
            stdin,
            buffer: Vec::new(),
        };
    }

    pub fn raw(&mut self) -> String {
        let mut string = String::new();
        self.stdin.read_line(&mut string).unwrap();
        return string;
    }

    pub fn skip(&mut self) {
        let mut string = String::new();
        self.stdin.read_line(&mut string).unwrap();
    }

    pub fn text(&mut self) -> String {
        return self.raw().trim().to_string();
    }

    pub fn next(&mut self) -> String {
        loop {
            match self.buffer.pop() {
                Some(token) => return token,
                None => {
                    self.buffer = self
                        .raw()
                        .split_whitespace()
                        .rev()
                        .map(String::from)
                        .collect();
                }
            };
        }
    }

    pub fn scalar<T>(&mut self) -> T
    where
        T: FromStr,
    {
        return self.next().parse().ok().unwrap();
    }

    pub fn vector<T>(&mut self, n: usize) -> Vec<T>
    where
        T: FromStr,
    {
        return (0..n).map(|_| self.scalar()).collect();
    }
}
