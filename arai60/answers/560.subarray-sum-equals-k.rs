/*
この問題で鍛えること:
- アルゴリズム: prefix sum
- データ構造: HashMap で prefix の出現回数を持つ
- Rust での練習ポイント: `HashMap::from`、`entry` 更新、負数を含むケースの考え方
- コード設計: 存在判定と個数管理の違いをコードで表す

問題の本質:
- 区間和 `sum[i..j]` は `prefix[j] - prefix[i]`
- 現在の prefix が `p` なら、和 `k` の区間数は「過去に `p - k` が何回出たか」で決まる
- したがって必要なのは prefix の存在ではなく、出現回数

解法候補:
1. 全区間を試す
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 素朴だが再利用がない
2. prefix sum + count HashMap
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 本命。負数があっても壊れない
3. 整理版
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: prefix 更新と回数加算を分けて読む

採用解法の説明:
- `count[prefix] = その prefix が出た回数` を持つ
- 現在 prefix を `p` とすると、`p - k` が過去に `c` 回出ていれば、`c` 個の区間が新たに完成する
- その後で現在 prefix `p` の出現回数を 1 増やす
- 初期値として `count[0] = 1` を入れると、先頭から始まる区間も同じ式で数えられる

Rust観点の解説:
- `HashMap::from([(0, 1)])` で初期 prefix を明示できる
- `count.get(...).copied().unwrap_or(0)` で未出現の prefix を 0 回として扱う
- `*count.entry(prefix).or_insert(0) += 1` が頻度更新の定番形

いいコード観点の解説:
- 「答えを増やす」と「prefix の出現回数を更新する」の順序が重要
- 先に現在 prefix を登録すると、長さ 0 の区間を誤って数える危険がある
- 整理版で更新順序を helper に閉じ込めると、意図が固定しやすい

落とし穴:
- sliding window を使ってしまう。負数があると成立しない
- prefix の存在だけ持って、個数を数え損ねる
- `count[0] = 1` を入れ忘れて、先頭始まりの区間を落とす
*/

use std::collections::HashMap;

struct Solution;
struct BruteForceSolution;
struct RefinedSolution;

impl BruteForceSolution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut answer = 0;

        for left in 0..nums.len() {
            let mut sum = 0;
            for &value in nums.iter().skip(left) {
                sum += value;
                if sum == k {
                    answer += 1;
                }
            }
        }

        answer
    }
}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::from([(0, 1)]);
        let mut prefix = 0;
        let mut answer = 0;

        for value in nums {
            prefix += value;
            answer += count.get(&(prefix - k)).copied().unwrap_or(0);
            *count.entry(prefix).or_insert(0) += 1;
        }

        answer
    }
}

impl RefinedSolution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::from([(0, 1)]);
        let mut prefix = 0;
        let mut answer = 0;

        for value in nums {
            prefix += value;
            answer += Self::completed_ranges(&count, prefix, k);
            Self::record_prefix(&mut count, prefix);
        }

        answer
    }

    fn completed_ranges(count: &HashMap<i32, i32>, prefix: i32, k: i32) -> i32 {
        count.get(&(prefix - k)).copied().unwrap_or(0)
    }

    fn record_prefix(count: &mut HashMap<i32, i32>, prefix: i32) {
        *count.entry(prefix).or_insert(0) += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, k: i32, expected: i32) {
        assert_eq!(BruteForceSolution::subarray_sum(nums.clone(), k), expected);
        assert_eq!(Solution::subarray_sum(nums.clone(), k), expected);
        assert_eq!(RefinedSolution::subarray_sum(nums, k), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![1, 1, 1], 2, 2);
    }

    #[test]
    fn negative_values_case() {
        assert_all_solutions(vec![1, -1, 0], 0, 3);
    }

    #[test]
    fn multiple_matches_case() {
        assert_all_solutions(vec![1, 2, 3], 3, 2);
    }
}
