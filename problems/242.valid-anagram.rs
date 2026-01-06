use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count: HashMap<char, i32> = HashMap::new();

        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            *count.entry(c).or_insert(0) -= 1;
            if count[&c] < 0 {
                return false;
            }
        }

        true
    }
}
