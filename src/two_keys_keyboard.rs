pub struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut operations = 0;
        let mut num_caracters = 1;
        let mut copy_len = 0;

        if n < 2 {
            return operations;
        }

        while n > num_caracters {
            if n % num_caracters == 0 {
                copy_len = num_caracters;
                operations += 1;
            }

            num_caracters += copy_len;
            operations += 1;
        }

        operations
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_step {
        ($input:literal expected $output:literal) => {
            let res = Solution::min_steps($input);

            assert_eq!(res, $output);
        };
    }

    #[test]
    fn tests() {
        check_step!(9 expected 6);
        check_step!(5 expected 5);
        check_step!(4 expected 4);
        check_step!(3 expected 3);
        check_step!(1 expected 0);
    }
}
