pub struct Solution {}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let roman_values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let roman_labels = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut i_roman = 0;
        let mut res = String::new();

        while i_roman != roman_labels.len() {
            let r = (num / roman_values[i_roman]) as usize;
            num %= roman_values[i_roman];

            if r > 0 {
                res.push_str(&roman_labels[i_roman].to_string().repeat(r));
            }

            i_roman += 1;
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
    }
}
