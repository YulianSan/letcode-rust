use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let mut letters = HashMap::new();

        for c in s.chars() {
            letters.insert(c, 1);
        }

        letters.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_strange_printer {
        ($input:literal expected $output:literal) => {
            let res = Solution::strange_printer($input.to_string());

            assert_eq!(res, $output, "Expected: {}, Output: {}", $output, res);
        }
    }

    #[test]
    fn tests() {
        check_strange_printer!("abcabc" expected 5);
        check_strange_printer!("aaabbb" expected 2);
        check_strange_printer!("aba" expected 2);
    }
}
