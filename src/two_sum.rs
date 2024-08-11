use std::{collections::HashMap};

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_nums = HashMap::new();
        for i in 0..nums.len() {
            let over = target - nums[i];
            match hash_nums.get(&over) {
                Some(&index) => return vec![index as i32, i as i32],
                None => { 
                    hash_nums.insert(nums[i], i) 
                }
            };
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tests() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }

}
