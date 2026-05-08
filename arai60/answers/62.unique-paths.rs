/*
この問題で鍛えること:
- アルゴリズム: grid DP
- データ構造: 2 次元 / 1 次元 table
- Rust での練習ポイント: 配列初期化、状態圧縮
- コード設計: 再帰木を表へ潰す感覚

問題の本質:
- 各マスへの到達方法数は「上から来る方法数 + 左から来る方法数」
- 同じ部分問題が大量に重なるので、再帰より表計算が自然
- 依存が上と左だけなので 1 次元へ圧縮できる

解法候補:
1. memo DFS
   - 採用/非採用理由: 状態定義は見やすい
2. 2 次元 DP
   - 採用/非採用理由: 遷移が最も直感的
3. 1 次元 DP
   - 採用/非採用理由: 本命。依存が少ないので自然に圧縮できる

採用解法の説明:
- `dp[j]` を「現在行の j 列目へ来る方法数」とする
- 左から来る数は更新後の `dp[j - 1]`、上から来る数は更新前の `dp[j]`
- よって `dp[j] += dp[j - 1]` だけでよい

Rust観点の解説:
- `vec![1; n]` で最上段を一気に初期化できる
- 2 次元版を残すと、圧縮版の意味を読み解きやすい
- memo 版は `Vec<Vec<Option<i32>>>` で未計算状態を表現すると素直

いいコード観点の解説:
- 圧縮版は短いが、何を上書きしているかが分からないと危険
- まず 2 次元で意味を固め、次に依存関係を削る順序が学習しやすい
- 状態圧縮は「情報を捨てる」のではなく「不要な履歴を持たない」こと

落とし穴:
- 初期化を 0 にして全経路を消す
- `dp[j] += dp[j - 1]` の左右の意味を混同する
- `m == 1` や `n == 1` のケースを壊す
*/

struct Solution;
struct GridDpSolution;
struct MemoSolution;

impl MemoSolution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        fn dfs(i: usize, j: usize, memo: &mut [Vec<Option<i32>>]) -> i32 {
            if i == 0 || j == 0 {
                return 1;
            }
            if let Some(value) = memo[i][j] {
                return value;
            }
            let value = dfs(i - 1, j, memo) + dfs(i, j - 1, memo);
            memo[i][j] = Some(value);
            value
        }

        let mut memo = vec![vec![None; n as usize]; m as usize];
        dfs(m as usize - 1, n as usize - 1, &mut memo)
    }
}

impl GridDpSolution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![1; n as usize]; m as usize];
        for i in 1..m as usize {
            for j in 1..n as usize {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n];
        for _ in 1..m as usize {
            for j in 1..n {
                dp[j] += dp[j - 1];
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(m: i32, n: i32, expected: i32) {
        assert_eq!(MemoSolution::unique_paths(m, n), expected);
        assert_eq!(GridDpSolution::unique_paths(m, n), expected);
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn rectangular_grid() {
        assert_all_solutions(3, 7, 28);
    }

    #[test]
    fn single_row_or_column() {
        assert_all_solutions(1, 5, 1);
        assert_all_solutions(5, 1, 1);
    }
}
