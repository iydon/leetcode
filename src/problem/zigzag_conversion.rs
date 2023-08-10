pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let size = num_rows as usize;
        let mut rows = vec![String::new(); size];
        (0..size)
            .chain((1..size - 1).rev())
            .cycle()
            .zip(s.chars())
            .for_each(|(i, c)| rows[i].push(c));
        return rows.join("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = [
            ("PAHNAPLSIIGYIR", "PAYPALISHIRING", 3),
            ("PINALSIGYAHRPI", "PAYPALISHIRING", 4),
            ("A", "A", 1),
        ];
        for (ans, s, num_rows) in cases {
            assert_eq!(ans, Solution::convert(s.to_string(), num_rows));
        }
    }
}
