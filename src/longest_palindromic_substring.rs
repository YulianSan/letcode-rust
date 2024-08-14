use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut positions: HashMap<char, Box<Vec<usize>>> = HashMap::new();
        let mut polindrome = String::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(position) = positions.get(&c) {
                for p in position.iter() {

                    let possible_polindrome = &s[*p..=i];
                    let middle = ((possible_polindrome.len() as f64 / 2f64).floor()).max(0.0) as usize;
                    let mut middle_end = middle;

                    if possible_polindrome.len() % 2 != 0 {
                        middle_end += 1;
                    }

                    if polindrome.len() >= possible_polindrome.len() {
                        break;
                    }
                    
                    if possible_polindrome[..middle].chars().eq(possible_polindrome[middle_end..].chars().rev()) {
                        if polindrome.len() < possible_polindrome.len() {
                            polindrome = possible_polindrome.to_string();
                        }
                        break;
                    }
                }

                let mut new_position = (*position).clone();
                new_position.push(i);
                positions.insert(c, new_position);
                continue;
            }

            if polindrome.len() == 0 {
                polindrome.push(c);
            }
            positions.insert(c, Box::new(vec![i]));
        }


        polindrome
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_polindromic {
        ($string:literal match $polindromic:literal) => {
            let res = Solution::longest_palindrome(String::from($string));

            assert_eq!(res, $polindromic);
        };
    }

    #[test]
    fn tests() {
        check_polindromic!("babad" match "bab");
        check_polindromic!("cbbd" match "bb");
        check_polindromic!("a" match "a");
        check_polindromic!("bb" match "bb");
        check_polindromic!("abcba" match "abcba");
        check_polindromic!("aacabdkacaa" match "aca");
        check_polindromic!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabcaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" match "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }
}
