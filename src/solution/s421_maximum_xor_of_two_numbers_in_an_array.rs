pub struct Solution;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        // bruteforce is ok haha =)
        let mut max = 0;
        let mut nums = nums.clone();
        nums.sort();
        nums.dedup();

        for (i, x) in nums.iter().enumerate()  {
            for y in &nums[0..i] {
                max = max.max(x ^ y);
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(
            Solution::find_maximum_xor(vec![3,10,5,25,2,8]),
            28
        );
        assert_eq!(
            Solution::find_maximum_xor(vec![14,70,53,83,49,91,36,80,92,51,66,70]),
            127
        );
    }
}
