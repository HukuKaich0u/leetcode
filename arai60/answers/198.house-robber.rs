/*
この問題で鍛えること:
- アルゴリズム: dynamic programming
- データ構造: 配列 or 定数個の状態
- Rust での練習ポイント: 状態圧縮、slice helper
- コード設計: 「今取る/取らない」の 2 択を最小限の状態へ落とす力

問題の本質:
- 各家で選択肢は 2 つだけ: 今の家を取るか、取らないか
- 取るなら前の家は取れないので 2 つ前の最適値に依存する
- 取らないなら 1 つ前の最適値をそのまま引き継ぐ

解法候補:
1. 再帰全探索
   - 採用/非採用理由: 比較用にはなるが指数時間
2. DP 配列
   - 採用/非採用理由: 遷移を明示しやすい
3. 状態圧縮
   - 採用/非採用理由: 本命。依存が 2 個しかないため自然

採用解法の説明:
- `prev1` を 1 つ前までの最適値、`prev2` を 2 つ前までの最適値とする
- 新しい家 `value` に対して、最適値は `max(prev1, prev2 + value)`
- これを左から順に更新すればよい

Rust観点の解説:
- `for value in nums` で所有権ごと取り、シンプルに走査できる
- helper 版では `&[i32]` を受けると 213 番の再利用にもつながる
- DP 配列版を残しておくと、圧縮前の状態定義との対応が見えやすい

いいコード観点の解説:
- 圧縮版は短いが、まず DP 配列版で意味を固めてから見る方が学習効率がよい
- 状態名を `prev1/prev2` にすると依存関係が崩れにくい
- 「何を持てば十分か」を削る感覚が DP 力の核心

落とし穴:
- `prev1` と `prev2` の更新順を誤る
- 今取るケースで `prev1 + value` としてしまう
- 空配列ケースを落とす
*/

use std::cmp::max;

struct Solution;
struct DpArraySolution;
struct RecursiveSolution;

impl RecursiveSolution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn dfs(index: usize, nums: &[i32]) -> i32 {
            if index >= nums.len() {
                return 0;
            }
            max(dfs(index + 1, nums), nums[index] + dfs(index + 2, nums))
        }

        dfs(0, &nums)
    }
}

impl DpArraySolution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];
        for i in 2..=nums.len() {
            dp[i] = max(dp[i - 1], dp[i - 2] + nums[i - 1]);
        }
        dp[nums.len()]
    }
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut prev2 = 0;
        let mut prev1 = 0;

        for value in nums {
            let current = max(prev1, prev2 + value);
            prev2 = prev1;
            prev1 = current;
        }

        prev1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, expected: i32) {
        assert_eq!(RecursiveSolution::rob(nums.clone()), expected);
        assert_eq!(DpArraySolution::rob(nums.clone()), expected);
        assert_eq!(Solution::rob(nums), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![1, 2, 3, 1], 4);
    }

    #[test]
    fn prefers_non_adjacent_large_sum() {
        assert_all_solutions(vec![2, 7, 9, 3, 1], 12);
    }

    #[test]
    fn empty_case() {
        assert_all_solutions(vec![], 0);
    }
}
