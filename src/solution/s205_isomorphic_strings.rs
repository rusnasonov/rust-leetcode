pub struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;

        let mut m1 = HashMap::<char, usize>::new();
        let mut m2 = HashMap::<char, usize>::new();
        for (i, (ch1, ch2)) in s.chars().zip(t.chars()).enumerate() {
            if m1.insert(ch1, i) != m2.insert(ch2, i) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic() {
        assert_eq!(
            Solution::is_isomorphic("egg".to_owned(), "add".to_owned()),
            true
        );
    }

    #[test]
    fn test_is_isomorphic_02() {
        assert_eq!(
            Solution::is_isomorphic("foo".to_owned(), "bar".to_owned()),
            false
        );
    }

    #[test]
    fn test_is_isomorphic_03() {
        assert_eq!(
            Solution::is_isomorphic("paper".to_owned(), "title".to_owned()),
            true
        );
    }
}
