use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = HashMap::<char, i32>::new();
        let chars: Vec<char> = s.chars().collect();

        for &c in &chars {
            *count.entry(c).or_insert(0) += 1;
        }

        for (i, &c) in chars.iter().enumerate() {
            if count.get(&c) == Some(&1) {
                return i as i32;
            }
        }

        -1
    }
}
