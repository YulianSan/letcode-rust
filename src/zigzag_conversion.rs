pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut direction: i32 = 1;
        let mut rows = vec![String::new(); num_rows as usize];
        let mut c_row: i32 = 0;

        if num_rows == 1 {
            return s;
        }

        for c in s.chars() {
            rows[c_row as usize].push(c);

            c_row += direction;

            if c_row == 0 || c_row == num_rows - 1 {
                direction *= -1;
            }
        }

        rows.iter().fold(String::new(), |mut acc, row| {
            acc.push_str(row);
            acc
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_zigzag {
        ($input:literal with $rows:literal expect $output:literal) => {
            let output = Solution::convert(String::from($input), $rows);
            assert_eq!(
                output, $output,
                "Expect output: {}, but received {}",
                $output, output
            );
        };
    }

    #[test]
    fn tests() {
        check_zigzag!("PAYPALISHIRING" with 4 expect "PINALSIGYAHRPI");
        check_zigzag!("PAYPALISHIRING" with 3 expect "PAHNAPLSIIGYIR");
        check_zigzag!("A" with 1 expect "A");
        check_zigzag!("ABCD" with 3 expect "ABDC");
        check_zigzag!("AB" with 1 expect "AB");
    }
}
