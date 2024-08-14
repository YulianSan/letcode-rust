pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let letter_by_zig = (num_rows * 2 - 2).max(num_rows);
        let mut zigzag_string = String::new();
        let chars = s.chars();
        let len = s.len();
        let cols = len.div_ceil(letter_by_zig as usize) as i32;

        for row in 0..num_rows {
            for col in 0..cols {
                let current_letter = (col * letter_by_zig) + row;
                let current_letter_back = ((col + 1) * letter_by_zig) - row;

                if (current_letter as usize) < s.len() {
                    if let Some(c) = chars.clone().skip((current_letter) as usize).next() {
                        zigzag_string.push(c);
                    }
                }

                if current_letter_back != current_letter && row != 0 && num_rows - 1 != row {
                    if let Some(c) = chars.clone().skip((current_letter_back) as usize).next() {
                        zigzag_string.push(c)
                    }
                }
            }
        }

        zigzag_string
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
    }
}
