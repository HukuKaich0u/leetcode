/*
この問題で鍛えること:
- アルゴリズム: circular DP
- データ構造: slice / 定数個の状態
- Rust での練習ポイント: slice 分割、helper 抽象化
- コード設計: 円環制約を直線問題へ分解する力

問題の本質:
- 198 番と同じだが、最初と最後が隣接する
- よって「最初を使うケース」と「最初を使わないケース」を同時には持てない
- したがって問題は 2 本の直線 problem に分解できる

解法候補:
1. 円環のまま再帰全探索
   - 採用/非採用理由: 遅い
2. 2 本の House Robber に帰着
   - 採用/非採用理由: 本命
3. helper で直線版を再利用
   - 採用/非採用理由: 整理版として最も自然

採用解法の説明:
- 解は必ず次のどちらかに属する
  - 先頭を使わない: `nums[1..]`
  - 末尾を使わない: `nums[..n-1]`
- この 2 ケースをそれぞれ 198 番の直線解法で解き、大きい方を返す

Rust観点の解説:
- `&nums[..nums.len()-1]` と `&nums[1..]` の slice 分割が自然
- helper へ `&[i32]` を渡すとコピー不要で再利用できる
- 小さい入力 `0/1` 件は先に `match` で切ると見通しがよい

いいコード観点の解説:
- 円環を直接 DP しようとするより、既知問題へ還元した方が説明も実装も安定する
- helper に直線ロジックを閉じ込めると、主関数はケース分けだけになる
- 「差分だけ見る」問題分解の良い練習になる

落とし穴:
- 先頭ケースと末尾ケースを同時に使ってしまう
- 長さ 1 のケースで slice 分割して panic する
- 198 番 helper をコピーしてバグを増やす
*/

use std::cmp::max;

struct Solution;
struct DpArraySolution;
struct RecursiveSolution;

impl RecursiveSolution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn dfs(index: usize, end: usize, nums: &[i32]) -> i32 {
            if index >= end {
                return 0;
            }
            max(dfs(index + 1, end, nums), nums[index] + dfs(index + 2, end, nums))
        }

        match nums.len() {
            0 => 0,
            1 => nums[0],
            n => max(dfs(0, n - 1, &nums), dfs(1, n, &nums)),
        }
    }
}

impl DpArraySolution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => max(rob_line_dp(&nums[..nums.len() - 1]), rob_line_dp(&nums[1..])),
        }
    }
}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            _ => max(rob_line(&nums[..nums.len() - 1]), rob_line(&nums[1..])),
        }
    }
}

fn rob_line(nums: &[i32]) -> i32 {
    let mut prev2 = 0;
    let mut prev1 = 0;
    for &value in nums {
        let current = max(prev1, prev2 + value);
        prev2 = prev1;
        prev1 = current;
    }
    prev1
}

fn rob_line_dp(nums: &[i32]) -> i32 {
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
        assert_all_solutions(vec![2, 3, 2], 3);
    }

    #[test]
    fn chooses_best_excluding_first_or_last() {
        assert_all_solutions(vec![1, 2, 3, 1], 4);
    }

    #[test]
    fn single_house_case() {
        assert_all_solutions(vec![5], 5);
    }
}
