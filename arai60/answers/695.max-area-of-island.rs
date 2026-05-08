/*
この問題で鍛えること:
- アルゴリズム: flood fill with aggregation
- データ構造: 2 次元 grid
- Rust での練習ポイント: DFS の返り値で情報を集約する
- コード設計: 200 番の「個数」から「成分サイズ最大」への拡張

問題の本質:
- 200 番との違いは、各成分を見つけたあと「何マスあったか」を返す点だけ
- つまり flood fill の返り値を `area` に変えればよい

解法候補:
1. 再帰 DFS
2. queue BFS
3. stack DFS

採用解法の説明:
- 陸に着いたら自分自身で 1 マス
- 4 方向へ再帰し、その返り値を全部足す
- 各始点で得た面積の最大値を更新する

Rust観点の解説:
- `dfs` が `i32` を返す形にすると集約ロジックが明快
- grid を 0 へ潰して visited 兼用にする
- BFS 別解を併記すると、集約対象が count でも traversal は変わらないと分かる

いいコード観点の解説:
- 200 番との差分を最小変更で表現できると理解が深い
- 再帰の返り値に何を乗せるかで問題の変種に対応できる
- 「探索」と「集約」を同じ helper の中に閉じ込めるのが自然

落とし穴:
- 面積を数える前に訪問済み化して二重計上を防げない
- 最大値更新を忘れる
- 斜め隣接を含める
*/

use std::collections::VecDeque;

struct Solution;
struct BfsSolution;
struct StackSolution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], r: i32, c: i32) -> i32 {
            if r < 0 || c < 0 || r as usize >= grid.len() || c as usize >= grid[0].len() {
                return 0;
            }
            if grid[r as usize][c as usize] == 0 {
                return 0;
            }
            grid[r as usize][c as usize] = 0;
            1 + dfs(grid, r + 1, c) + dfs(grid, r - 1, c) + dfs(grid, r, c + 1) + dfs(grid, r, c - 1)
        }

        let mut answer = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                answer = answer.max(dfs(&mut grid, r as i32, c as i32));
            }
        }
        answer
    }
}

impl BfsSolution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 0 {
                    continue;
                }
                grid[r][c] = 0;
                let mut area = 0;
                let mut queue = VecDeque::from([(r, c)]);
                while let Some((row, col)) = queue.pop_front() {
                    area += 1;
                    for (nr, nc) in neighbors(row, col, grid.len(), grid[0].len()) {
                        if grid[nr][nc] == 1 {
                            grid[nr][nc] = 0;
                            queue.push_back((nr, nc));
                        }
                    }
                }
                answer = answer.max(area);
            }
        }
        answer
    }
}

impl StackSolution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 0 {
                    continue;
                }
                let mut area = 0;
                let mut stack = vec![(r, c)];
                grid[r][c] = 0;
                while let Some((row, col)) = stack.pop() {
                    area += 1;
                    for (nr, nc) in neighbors(row, col, grid.len(), grid[0].len()) {
                        if grid[nr][nc] == 1 {
                            grid[nr][nc] = 0;
                            stack.push((nr, nc));
                        }
                    }
                }
                answer = answer.max(area);
            }
        }
        answer
    }
}

fn neighbors(r: usize, c: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if r > 0 {
        out.push((r - 1, c));
    }
    if r + 1 < rows {
        out.push((r + 1, c));
    }
    if c > 0 {
        out.push((r, c - 1));
    }
    if c + 1 < cols {
        out.push((r, c + 1));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(grid: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(Solution::max_area_of_island(grid.clone()), expected);
        assert_eq!(BfsSolution::max_area_of_island(grid.clone()), expected);
        assert_eq!(StackSolution::max_area_of_island(grid), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(
            vec![
                vec![0, 0, 1, 0, 0],
                vec![1, 1, 1, 0, 1],
                vec![0, 1, 0, 0, 1],
                vec![0, 0, 0, 1, 1],
            ],
            5,
        );
    }

    #[test]
    fn no_land_case() {
        assert_all_solutions(vec![vec![0, 0], vec![0, 0]], 0);
    }
}
