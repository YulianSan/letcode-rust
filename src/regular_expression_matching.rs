pub struct Solution;

#[derive(Debug)]
enum CharRegex {
    Any { mul: Multiple },
    Char { char: char, mul: Multiple },
}

#[derive(Debug)]
enum Multiple {
    Yes,
    No,
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut regex_chars: Vec<CharRegex> = vec![];
        if s == p {
            return true;
        }

        let bytes = s.as_bytes();

        let p_chars = p.chars();
        let mut mul = Multiple::No;

        for c in p_chars.rev() {
            match c {
                '*' => mul = Multiple::Yes,
                '.' => {
                    regex_chars.push(CharRegex::Any { mul });
                    mul = Multiple::No;
                }
                c => {
                    regex_chars.push(CharRegex::Char { char: c, mul });
                    mul = Multiple::No;
                }
            }
        }

        regex_chars.reverse();

        let mut current_char = 0;
        let mut current_regex = 0;

        while regex_chars.len() > current_regex {
            if let Some(c_s) = bytes.get(current_char) {
                match &regex_chars[current_regex] {
                    CharRegex::Char {
                        char,
                        mul: Multiple::No,
                    } if *char as u8 != *c_s => {
                        return false;
                    }
                    CharRegex::Char {
                        char,
                        mul: Multiple::Yes,
                    } if *char as u8 != *c_s => {
                        current_regex += 1;
                    }
                    CharRegex::Char {
                        char,
                        mul: Multiple::Yes,
                    } if *char as u8 == *c_s => {
                        current_char += 1;
                    }
                    CharRegex::Char {
                        char: _,
                        mul: Multiple::No,
                    } => {
                        current_regex += 1;
                        current_char += 1;
                    }
                    CharRegex::Any { mul: Multiple::Yes } => {
                        if let Some(c_n) = regex_chars.get(current_regex + 1) {
                            match c_n {
                                CharRegex::Any { mul: _ } => {
                                    current_char += 1;
                                    current_regex += 1;
                                    continue;
                                }
                                CharRegex::Char { char, mul: _ } => {
                                    if (*char as u8) == *c_s {
                                        current_char += 1;
                                        current_regex += 1;
                                        continue;
                                    }
                                }
                            }
                        }

                        current_char += 1;
                    }
                    CharRegex::Any { mul: Multiple::No } => {
                        current_regex += 1;
                    }
                    mul => {
                        dbg!(mul);
                    }
                }
            } else {
                return bytes.len() < current_char
            }
        }

        bytes.len() <= current_char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_match {
        ($regex:literal valid $str:literal) => {
            assert_eq!(
                Solution::is_match($str.to_string(), $regex.to_string()),
                true,
                "Invalid match, str: {}, regex: {}",
                $str,
                $regex
            );
        };
        ($regex:literal invalid $str:literal) => {
            assert_eq!(
                Solution::is_match($str.to_string(), $regex.to_string()),
                false,
                "Valid match, str: {}, regex: {}",
                $str,
                $regex
            );
        };
    }

    #[test]
    fn tests() {
        check_match!("ab*a" invalid "aaa");
        check_match!("c*a*b" valid "aab");
        check_match!("a" invalid "aa");
        check_match!("aa" invalid "a");
        check_match!("a*" valid "aa");
        // check_match!("a*b" valid "aab");
    }
}
