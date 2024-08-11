use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_position_repeat = 0;
        let mut max_len = 0;
        let mut positions: HashMap<char, i32> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            if let Some(last_position_char) = positions.get(&c) {
                // sum more one to remove repeat char
                last_position_repeat = last_position_repeat.max(*last_position_char + 1);
            }

            positions.insert(c, i as i32);
            max_len = max_len.max(i as i32 + 1 - last_position_repeat);
        }
        
        max_len
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! length_of_longest {
        ($string:literal is $expected:expr) => {
            assert_eq!(Solution::length_of_longest_substring($string.to_string()), $expected);
        }
    }

    #[test]
    fn tests() {
        length_of_longest!("abcabcbb" is 3);
        length_of_longest!("bbbbb" is 1);
        length_of_longest!("pwwkew" is 3);
        length_of_longest!(" " is 1);
        length_of_longest!("au" is 2);
    }
}
