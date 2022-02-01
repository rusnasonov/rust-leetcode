use std::collections::{HashMap};

pub struct Trie {
    pub  value: Option<char>,
    pub is_final: bool,
    pub  childs: HashMap<char, Trie>
}

impl Trie {
    pub fn new(value: Option<char>, is_final: bool) -> Self {
        Trie{
            value,
            is_final,
            childs: HashMap::new()
        }
    }
}

struct WordDictionary {
    root: Trie
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl WordDictionary {
    fn new() -> Self {
        Self{
            root: Trie::new(
                None,
                false
            )
        }
    }

    fn add_word(&mut self, word: String) {
        let mut current = &mut self.root;
        for c in word.chars() {
            current = current
                .childs
                .entry(c)
                .or_insert(
                    Trie::new(
                        Some(c),
                        false
                    )
                );
        }
        current.is_final = true
    }

    fn dfs(&self, root: &Trie, word: &Vec<char>, idx: usize) -> bool {
        let mut current = root;
        let mut i = idx;
        while i < word.len() {
            let c = word[i];
            if c == '.' {
                for (_, next) in &current.childs {
                    if self.dfs(&next, word, i+1) {
                        return true
                    }
                }
                return false;
            }
            let entry = current.childs.get(&c);
            match entry {
                None => return false,
                Some(v) => current = v
            }
            i+=1;
        }
        current.is_final
    }

    fn search(&self, word: String) -> bool {
        let chars = word.chars().collect();
        self.dfs(&self.root, &chars, 0)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut word_dict = WordDictionary::new();
        for word in vec!["bad","dad","mad"] {
            word_dict.add_word(word.to_string())
        }
        for case in vec![("pad", false), ("bad", true), (".ad", true), ("b..", true)] {
            assert_eq!(
                word_dict.search(case.0.to_string()),
                case.1,
                "case {:?}",
                case
            )
        }
    }
}
