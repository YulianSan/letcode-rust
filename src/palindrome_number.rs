pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }

        if x < 10 {
            return true;
        }

        let mut nums: Vec<i32> = vec![];

        while x > 0 {
            nums.push(x % 10);
            x = x/10;
        }

        let nums_len = nums.len() - 1;

        for i in 0..=nums_len/2 {
            if nums[i] != nums[nums_len - i]{
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_is_palindrome {
        ($input:literal is polidrome) => {
            assert_eq!(Solution::is_palindrome($input), true);
        };
        ($input:literal isnt polidrome) => {
            assert_eq!(Solution::is_palindrome($input), false);
        }
    }

    #[test]
    fn tests() {
        check_is_palindrome!(0 is polidrome);
        check_is_palindrome!(10 isnt polidrome);
        check_is_palindrome!(121 is polidrome);
        check_is_palindrome!(-121 isnt polidrome);
    }
}
