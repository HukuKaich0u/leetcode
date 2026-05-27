// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N)
  空間計算量:
*/

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = HashMap::<char, i32>::new();
        let chars = s.chars().collect::<Vec<char>>();

        for &character in &chars {
            *count.entry(character).or_insert(0) += 1;
        }

        for (index, &character) in chars.iter().enumerate() {
            if count.get(&character) == Some(&1) {
                return index as i32;
            }
        }

        -1
    }
}
