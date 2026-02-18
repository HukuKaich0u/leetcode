// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n)
  空間計算量:
*/

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.into_iter().collect::<HashSet<_>>();
        let mut intersection_set = HashSet::new();

        for element in nums2 {
            if nums1_set.contains(&element) {
                intersection_set.insert(element);
            }
        }

        intersection_set.into_iter().collect::<Vec<_>>()
    }
}
