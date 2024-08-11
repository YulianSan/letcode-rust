pub struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut i = (0, 0);
        let mut sorted_array = vec![];
        while nums1.len() > i.0 && nums2.len() > i.1 {
            if nums1[i.0] < nums2[i.1] {
                sorted_array.push(nums1[i.0]);
                i.0 += 1;
            } else {
                sorted_array.push(nums2[i.1]);
                i.1 += 1;
            }
        }

        while nums1.len() > i.0 {
            sorted_array.push(nums1[i.0]);
            i.0 += 1;
        }

        while nums2.len() > i.1 {
            sorted_array.push(nums2[i.1]);
            i.1 += 1;
        }

        let mid_index = sorted_array.len() / 2;
        if sorted_array.len() % 2 == 0 {
            let sum =
                sorted_array[mid_index] + sorted_array[mid_index - 1];
            (sum as f64) / 2f64
        } else {
            sorted_array[mid_index] as f64
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_sum_mid_arrays {
        ([$($v1:expr),+] + [$($v2:expr),+] matches $r:expr) => {
            let v1 = vec![$($v1),*];
            let v2 = vec![$($v2),*];
            let res = Solution::find_median_sorted_arrays(v1, v2);
            assert_eq!(
                res, $r as f64,
                "Return solution: {} | Expected: {}", res, $r 
            );
        }
    }

    #[test]
    fn tests() {
        check_sum_mid_arrays!([1, 3] + [2] matches 2);
        check_sum_mid_arrays!([1, 2] + [3, 4] matches 2.5);
    }
}
