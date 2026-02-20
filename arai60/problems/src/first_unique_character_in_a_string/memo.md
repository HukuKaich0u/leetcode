# 参考コード

- Iwasaさん
```rust
use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        const NOT_FOUND: i32 = -1;
        const INVALID_INPUT: i32 = -2;
        if !s.chars().any(|c| c.is_ascii_lowercase()) {
            return INVALID_INPUT;
        }
        let mut char_index_map = HashMap::<char, (usize, usize)>::with_capacity(26);
        for (index, c) in s.chars().enumerate() {
            let (_, cnt) = char_index_map.entry(c).or_insert((index, 0));
            *cnt += 1;
        }
        char_index_map
            .iter()
            .filter_map(
                |(_, (index, cnt))| {
                    if *cnt == 1 {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
            .min()
            .map(|index| *index as i32)
            .unwrap_or(NOT_FOUND)
    }
}
```

- t9a-devさん
```rust
use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count_by_character: HashMap<char, usize> = HashMap::new();

        for c in s.chars().into_iter() {
            count_by_character
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for (i, c) in s.chars().enumerate() {
            if count_by_character.get(&c).is_some_and(|count| *count == 1) {
                return i.try_into().unwrap_or(-1);
            }
        }

        -1
    }
}
```
