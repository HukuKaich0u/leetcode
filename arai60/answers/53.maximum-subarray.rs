/*
この問題で鍛えること:
- アルゴリズム: Kadane 法と状態圧縮
- データ構造: prefix sum による別視点
- Rust での練習ポイント: 走査中の状態更新、`max` の使い方、補助関数化
- コード設計: 全探索から状態要約へ落とす感覚

問題の本質:
- 欲しいのは「全区間の和」だが、次の位置で必要なのは全区間の情報ではない
- 各位置で必要なのは「その位置で終わる最大部分配列の和」だけ
- つまり大きな DP テーブルを持つ問題ではなく、直前状態だけを持てば十分な問題

解法候補:
1. 全区間を試す
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 定義確認にはよいが、状態圧縮が見えない
2. Kadane 法
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 最も本質的で、本命
3. prefix sum と最小 prefix
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 同じ O(n) でも別の見え方を与えてくれる

採用解法の説明:
- `current` を「現在位置で終わる最大和」とする
- 次の値 `value` を足すとき、選択肢は 2 つだけ
  - ここまでの区間に `value` をつなぐ
  - `value` から新しく区間を始める
- よって `current = max(value, current + value)` になる
- 各位置での `current` の最大値を全体の答え `best` として持てばよい

Rust観点の解説:
- `nums.iter().skip(1)` で先頭を初期値に使ったあと、残りだけ走査する
- `current` と `best` を `i32` のまま持ち、不要な型変換を避ける
- 整理版では「次状態の計算」を補助関数化して、状態遷移だけ切り出せる

いいコード観点の解説:
- `current` は「ここで終わる最大値」、`best` は「全体最大値」と役割が明確
- 本命実装は 1 メソッドで完結した方が、状態遷移が見えやすい
- 一方で、整理版は DP の更新則を独立した概念として見せたいときに有効

落とし穴:
- `current` を「これまでの最大値」と混同する
- 全負数ケースで 0 を返してしまう
- Kadane 法を丸暗記して、なぜ新しく始めてよいかを説明できない
*/

use std::cmp::max;

struct Solution;
struct BruteForceSolution;
struct PrefixMinSolution;
struct RefinedKadaneSolution;

impl BruteForceSolution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best = i32::MIN;

        for left in 0..nums.len() {
            let mut sum = 0;
            for &value in nums.iter().skip(left) {
                sum += value;
                best = best.max(sum);
            }
        }

        best
    }
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current = nums[0];
        let mut best = nums[0];

        for &value in nums.iter().skip(1) {
            current = max(value, current + value);
            best = max(best, current);
        }

        best
    }
}

impl PrefixMinSolution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = 0;
        let mut min_prefix = 0;
        let mut best = i32::MIN;

        for value in nums {
            prefix_sum += value;
            best = best.max(prefix_sum - min_prefix);
            min_prefix = min_prefix.min(prefix_sum);
        }

        best
    }
}

impl RefinedKadaneSolution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current = nums[0];
        let mut best = nums[0];

        for &value in nums.iter().skip(1) {
            current = Self::next_current(current, value);
            best = best.max(current);
        }

        best
    }

    fn next_current(current: i32, value: i32) -> i32 {
        max(value, current + value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, expected: i32) {
        assert_eq!(BruteForceSolution::max_sub_array(nums.clone()), expected);
        assert_eq!(Solution::max_sub_array(nums.clone()), expected);
        assert_eq!(PrefixMinSolution::max_sub_array(nums.clone()), expected);
        assert_eq!(RefinedKadaneSolution::max_sub_array(nums), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4], 6);
    }

    #[test]
    fn all_negative_values() {
        assert_all_solutions(vec![-3, -2, -5], -2);
    }

    #[test]
    fn single_element() {
        assert_all_solutions(vec![5], 5);
    }
}
