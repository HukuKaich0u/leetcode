(Yoshiki Iwasaさん)
```rust
pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliment_map = HashMap::<i32, usize>::new();

        for (index, n) in nums.iter().enumerate() {
            let compliment = target - *n;
            match compliment_map.get(&compliment) {
                Some(compliment_index) => return vec![index as i32, *compliment_index as i32],
                None => {
                    compliment_map.insert(*n, index);
                }
            }
        }
        eprintln!(
            "No pairs can make the target. nums: {:?}, target: {:?}",
            nums, target
        );
        vec![]
    }
}
```
(t9a-devさん)
```rust
use std::collections::HashMap;

/*
  時間計算量: O(n)
  空間計算量: O(n)
*/
pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_hash_map: HashMap<i32, usize> = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            let complement = target - num;
            if let Some(complement_index) = nums_hash_map.get(&complement) {
                return vec![
                    // clone()でも良いが、to_owned()の方がやりたいこと（所有権が欲しい）と一致しているので
                    complement_index.to_owned().try_into().unwrap(),
                    i.try_into().unwrap(),
                ];
            }
            nums_hash_map.insert(num, i);
        }
        vec![]
    }
}
```
