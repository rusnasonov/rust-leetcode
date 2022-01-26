pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        let u = n as usize;
        let mut nums = vec![true; u];
        nums[0] = false;
        nums[1] = false;
        let mut i = 2;
        while i*i < u {
            if nums[i] == true {
                let mut j = i*i;
                while j < u {
                    nums[j] = false;
                    j+=i;
                }
            }
            i+=1;
        }
        nums.iter().filter(|&i| *i == true).count() as i32
    }
}

#[test]
fn simple(){
    assert_eq!(Solution::count_primes(10), 4);
    assert_eq!(Solution::count_primes(0), 0);
    assert_eq!(Solution::count_primes(1), 0);
}
