pub struct Solution;

#[derive(Debug)]
enum Operators {
    Sum,
    Sub,
}

impl Operators {
    pub fn to_char(c: char) -> Option<Operators> {
        match c {
            '+' => Some(Operators::Sum),
            '-' => Some(Operators::Sub),
            _ => None,
        }
    }
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut num: i32 = 0;
        let mut operators: Option<Operators> = None;
        let mut operator_insert = false;
        let mut init_num = false;

        for c in s.chars() {
            if let Some(n) = c.to_digit(10) {
                init_num = true;
                operator_insert = false;
                    num = match num.checked_mul(10) {
                    Some(mul) => match mul.checked_add(n as i32) {
                        Some(sum) => sum,
                        None => {
                            if let Some(operator) = operators {
                                match operator {
                                    Operators::Sub => return i32::MIN,
                                    _ => return i32::MAX,
                                }
                            } else if num > 0 {
                                return i32::MAX
                            } else {
                                return i32::MIN
                            }
                        }
                    },
                    None => {
                        if let Some(operator) = operators {
                            match operator {
                                Operators::Sub => return i32::MIN,
                                _ => return i32::MAX,
                            }
                        } else if num > 0 {
                            return i32::MAX;
                        } else {
                            return i32::MIN;
                        }
                    }
                };
            } else if let Some(operator) = Operators::to_char(c) {
                if init_num == true {
                    break;
                }

                if operator_insert {
                    return 0;
                }
                operator_insert = true;
                operators = Some(operator);
            } else if c != ' ' {
                break;
            } else {
                if init_num || operator_insert {
                    break;
                }
            }
        }

        match num.checked_mul(match operators {
            Some(Operators::Sub) => -1,
            _ => 1
        }) {
            Some(num) => num,
            None => match operators {
                Some(Operators::Sub) => i32::MIN,
                _ => i32::MAX
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_atoi {
        ($input:literal result $output:literal) => {
            let res = Solution::my_atoi($input.to_string());

            assert_eq!(res, $output, "Input: {}, Expected {}, Returned: {}", $input, $output, res);
        };
    }

    #[test]
    fn tests() {
        check_atoi!("    +11191657170" result 2147483647);
        check_atoi!("  +  413" result 0);
        check_atoi!("123-" result 123);
        check_atoi!("-5-" result -5);
        check_atoi!("+-12" result 0);
        check_atoi!("0  123" result 0);
        check_atoi!("   -042" result -42);
        check_atoi!("  0000000000012345678" result 12345678);
        check_atoi!("-91283472332" result -2147483648);
        check_atoi!("21474836460" result 2147483647);
        check_atoi!("0-1" result 0);
        check_atoi!("42" result 42);
        check_atoi!("-042" result -42);
        check_atoi!("1337c0d3" result 1337);
        check_atoi!("words and 987" result 0);
        check_atoi!("1.1 - 1" result 1);
        check_atoi!("+0 123" result 0);
    }
}
