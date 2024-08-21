pub struct Solution;

#[derive(Debug)]
enum CharRegex {
    Any { mul: bool },
    Char { char: char, mul: bool },
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut regex_chars: Vec<CharRegex> = vec![];
        if s == p {
            return true;
        }

        let bytes = s.as_bytes();

        let p_chars = p.chars();
        let mut next_mul = false;

        for c in p_chars.rev() {
            if let Some(c_p) = regex_chars.last() {
                if let CharRegex::Char { char, mul } = c_p {
                    if *char == c && *mul {
                        continue;
                    }
                }
            }
            match c {
                '*' => next_mul = true,
                '.' => {
                    regex_chars.push(CharRegex::Any { mul: next_mul });
                    next_mul = false;
                }
                c => {
                    regex_chars.push(CharRegex::Char {
                        char: c,
                        mul: next_mul,
                    });
                    next_mul = false;
                }
            }
        }

        regex_chars.reverse();

        let mut current_char_used = false;
        let mut current_char = 0;
        let mut current_regex = 0;

        while regex_chars.len() > current_regex && bytes.len() > current_char {
            if let Some(c_s) = bytes.get(current_char) {
                match regex_chars[current_regex] {
                    CharRegex::Char { char, mul } => {
                        if *c_s == char as u8 {
                            if mul {
                                current_char_used = true;
                            } else {
                                current_regex += 1;
                            }
                            current_char += 1;
                        } else {
                            current_regex += 1;
                            current_char_used = false;
                        }
                    }
                    CharRegex::Any { mul } => {
                        if mul && current_char_used {
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
                        }

                        if mul {
                            current_char_used = true;
                        } else {
                            current_regex += 1;
                        }

                        current_char += 1;
                    }
                }
            } else {
                return false;
            }
        }

        if regex_chars.len() < bytes.len() {
            current_char == bytes.len()
        } else {
            current_regex == regex_chars.len()
        }
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
        check_match!("ab*a" valid "aaa");
        check_match!("c*a*b" valid "aab");
        check_match!("a" invalid "aa");
        check_match!("aa" invalid "a");
        check_match!("a*" valid "aa");
        check_match!("a*b" valid "aab");
    }
}
