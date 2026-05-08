/*
この問題で鍛えること:
- アルゴリズム: 最小化 DP
- データ構造: 1 次元 table
- Rust での練習ポイント: sentinel 値、`usize` index 変換、helper 化
- コード設計: 到達不能状態をどう表すか

問題の本質:
- `dp[a]` を「金額 `a` を作る最小枚数」と置く
- 最後に使うコインが `coin` なら、残り `a - coin` はすでに解けているはず
- よって遷移は `dp[a] = min(dp[a], dp[a-coin] + 1)`

解法候補:
1. 再帰全探索
   - 採用/非採用理由: 重複計算が多すぎる
2. bottom-up DP
   - 採用/非採用理由: 本命
3. memo DFS
   - 採用/非採用理由: 同じ本質を top-down で見る別解

採用解法の説明:
- `dp[0] = 0`、それ以外は大きな sentinel 値で初期化する
- 各金額 `a` に対して、使えるコインを全部試す
- 最後まで sentinel のままなら到達不能なので `-1`

Rust観点の解説:
- index は `usize` なので、金額を最初に `usize` へ変換する
- sentinel を `amount + 1` にすると「最悪でも amount 枚超は不要」という上界が使える
- memo 版では `Option<i32>` を使うと未計算状態を区別しやすい

いいコード観点の解説:
- 到達不能状態を `inf` で置くのは DP の標準パターン
- 本命実装はテーブル意味と遷移がそのままコードへ出ていて読みやすい
- top-down 別解を置くと、状態定義が共通であることが分かる

落とし穴:
- `dp[a - coin]` が未到達なのに `+1` してしまう
- `usize` 変換の前後で負値を想定し混乱する
- 到達不能時の `-1` を返し忘れる
*/

use std::cmp::min;

struct Solution;
struct DpArraySolution;
struct MemoSolution;

impl MemoSolution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        fn dfs(amount: i32, coins: &[i32], memo: &mut [Option<i32>]) -> i32 {
            if amount == 0 {
                return 0;
            }
            if amount < 0 {
                return i32::MAX / 2;
            }
            if let Some(value) = memo[amount as usize] {
                return value;
            }

            let mut best = i32::MAX / 2;
            for &coin in coins {
                best = min(best, 1 + dfs(amount - coin, coins, memo));
            }

            memo[amount as usize] = Some(best);
            best
        }

        let mut memo = vec![None; amount.max(0) as usize + 1];
        let answer = dfs(amount, &coins, &mut memo);
        if answer >= i32::MAX / 4 { -1 } else { answer }
    }
}

impl DpArraySolution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![amount as i32 + 1; amount + 1];
        dp[0] = 0;
        for a in 1..=amount {
            for &coin in &coins {
                let coin = coin as usize;
                if coin <= a {
                    dp[a] = min(dp[a], dp[a - coin] + 1);
                }
            }
        }
        if dp[amount] > amount as i32 { -1 } else { dp[amount] }
    }
}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        DpArraySolution::coin_change(coins, amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(coins: Vec<i32>, amount: i32, expected: i32) {
        assert_eq!(MemoSolution::coin_change(coins.clone(), amount), expected);
        assert_eq!(DpArraySolution::coin_change(coins.clone(), amount), expected);
        assert_eq!(Solution::coin_change(coins, amount), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![1, 2, 5], 11, 3);
    }

    #[test]
    fn unreachable_amount() {
        assert_all_solutions(vec![2], 3, -1);
    }

    #[test]
    fn zero_amount() {
        assert_all_solutions(vec![1], 0, 0);
    }
}
