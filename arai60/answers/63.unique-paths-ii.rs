/*
この問題で鍛えること:
- アルゴリズム: grid DP with obstacles
- データ構造: 1 次元 / 2 次元 table
- Rust での練習ポイント: 到達不能を 0 で表す DP
- コード設計: 前問の状態定義を保ったまま制約追加に対応する力

問題の本質:
- 62 番と状態定義は同じで、「そのマスへ来る方法数」を持てばよい
- 障害物があるマスだけは到達不能なので 0 にする
- 前問との差分を局所ルールで吸収できることが重要

解法候補:
1. memo DFS
2. 2 次元 DP
3. 1 次元 DP
   - 採用/非採用理由: 本命

採用解法の説明:
- 障害物マスなら `dp[col] = 0`
- そうでなければ、左から来る分 `dp[col - 1]` を足す
- `dp[col]` の元の値が上から来る分なので、62 番の圧縮版をそのまま拡張できる

Rust観点の解説:
- `dp[0] = 1` から始めると、開始地点が障害物でない限り自然に伝播する
- 障害物にぶつかったら `0` へリセットするだけで未到達を表現できる
- memo 版では grid を借用して index でたどるのが素直

いいコード観点の解説:
- 前問との差分が小さいときほど、状態定義を変えない方が強い
- 1 次元版は短いが、障害物処理の意味が読めるようコメントや解説で補う価値がある
- 問題差分を局所化できると再利用しやすいコードになる

落とし穴:
- 障害物を見ても `dp[col]` を 0 にしない
- 開始地点が障害物のケースを落とす
- 左と上のどちらの値を持っているか分からなくなる
*/

struct Solution;
struct GridDpSolution;
struct MemoSolution;

impl MemoSolution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(i: usize, j: usize, grid: &[Vec<i32>], memo: &mut [Vec<Option<i32>>]) -> i32 {
            if grid[i][j] == 1 {
                return 0;
            }
            if i == 0 && j == 0 {
                return 1;
            }
            if let Some(value) = memo[i][j] {
                return value;
            }

            let mut ways = 0;
            if i > 0 {
                ways += dfs(i - 1, j, grid, memo);
            }
            if j > 0 {
                ways += dfs(i, j - 1, grid, memo);
            }
            memo[i][j] = Some(ways);
            ways
        }

        let rows = obstacle_grid.len();
        let cols = obstacle_grid[0].len();
        let mut memo = vec![vec![None; cols]; rows];
        dfs(rows - 1, cols - 1, &obstacle_grid, &mut memo)
    }
}

impl GridDpSolution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let rows = obstacle_grid.len();
        let cols = obstacle_grid[0].len();
        let mut dp = vec![vec![0; cols]; rows];

        if obstacle_grid[0][0] == 0 {
            dp[0][0] = 1;
        }

        for i in 0..rows {
            for j in 0..cols {
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                    continue;
                }
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }

        dp[rows - 1][cols - 1]
    }
}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let cols = obstacle_grid[0].len();
        let mut dp = vec![0; cols];
        dp[0] = 1;

        for row in obstacle_grid {
            for col in 0..cols {
                if row[col] == 1 {
                    dp[col] = 0;
                } else if col > 0 {
                    dp[col] += dp[col - 1];
                }
            }
        }

        dp[cols - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(grid: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(MemoSolution::unique_paths_with_obstacles(grid.clone()), expected);
        assert_eq!(GridDpSolution::unique_paths_with_obstacles(grid.clone()), expected);
        assert_eq!(Solution::unique_paths_with_obstacles(grid), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 2);
    }

    #[test]
    fn blocked_start_has_no_path() {
        assert_all_solutions(vec![vec![1]], 0);
    }
}
