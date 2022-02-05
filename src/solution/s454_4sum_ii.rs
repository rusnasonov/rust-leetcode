use std::{collections::HashMap, ops::Add};

pub struct Solution{}

#[allow(dead_code)]
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut sums: HashMap<i32, usize> = HashMap::new();
        for i in &nums1 {
            for j in &nums2 {
                *sums.entry(i+j).or_insert(0) += 1;
            }
        }
        for i in &nums3 {
            for j in &nums4 {
                if let Some(v) = sums.get(&-(i+j)) {
                    count += v;
                }
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            Solution::four_sum_count(
                vec![1,2], vec![-2,-1], vec![-1,2], vec![0,2]
            ),
            2
        );
        assert_eq!(
            Solution::four_sum_count(
                vec![0], vec![0], vec![0], vec![0]
            ),
            1
        );
    }
}
