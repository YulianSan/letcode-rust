pub struct Solution;

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        //  I will fixed this
        let mut y: i64 = 0;
        while x != 0 {
            y = (y * 10) + (x as i64 % 10);
            x = x/10;
        }

        if (i32::MAX as i64) < y || (i32::MIN as i64) > y {
            return 0;
        }

        y as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_reverse {
        ($input:literal expected $output:literal) => {
            let res = Solution::reverse($input);
            assert_eq!(res, $output);
        }
    }

    #[test]
    fn tests() {
        check_reverse!(1534236469 expected 0);
        check_reverse!(900000 expected 9);
        check_reverse!(-120 expected -21);
    }
}
