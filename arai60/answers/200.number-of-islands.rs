/*
この問題で鍛えること:
- アルゴリズム: flood fill, DFS/BFS
- データ構造: 2 次元 grid
- Rust での練習ポイント: 2 次元 index、queue、再帰 helper
- コード設計: 「成分を 1 回見つけたら丸ごと潰す」発想

問題の本質:
- 未訪問の陸を見つけた瞬間、その陸を含む島が 1 つ確定する
- あとはその連結成分を全部訪問済みにすれば二重計上しない
- 個数問題では「数える瞬間」と「再訪防止」が核心

解法候補:
1. 再帰 DFS
2. queue BFS
3. Union Find
   - 採用/非採用理由: 本問では DFS/BFS の方が直接的

採用解法の説明:
- 全マスをなめて、`'1'` を見つけたら答えを 1 増やす
- 同時に DFS でその島の `'1'` を全部 `'0'` へ変えて潰す
- これで次に同じ島を数えることはない

Rust観点の解説:
- grid を破壊的に更新する方が `visited` 配列を別に持たずに済む
- BFS 版では `VecDeque` が自然
- 境界チェックは helper へ閉じ込めると主処理が読みやすい

いいコード観点の解説:
- flood fill は「見つけた瞬間に count、探索は再訪防止のため」という役割分担が大事
- DFS/BFS の両方を置くと、本質は traversal ではなく成分処理だと分かる
- 問題の状態を破壊してよいなら、それを使う方がコードは簡潔になる

落とし穴:
- 訪問済み化を忘れて同じ島を何回も数える
- 斜め方向までつないでしまう
- 行列境界チェックを 1 箇所でも漏らす
*/

use std::collections::VecDeque;

struct Solution;
struct BfsSolution;
struct UnionFindSolution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut [Vec<char>], r: i32, c: i32) {
            if r < 0 || c < 0 || r as usize >= grid.len() || c as usize >= grid[0].len() {
                return;
            }
            if grid[r as usize][c as usize] != '1' {
                return;
            }
            grid[r as usize][c as usize] = '0';
            dfs(grid, r + 1, c);
            dfs(grid, r - 1, c);
            dfs(grid, r, c + 1);
            dfs(grid, r, c - 1);
        }

        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '1' {
                    count += 1;
                    dfs(&mut grid, r as i32, c as i32);
                }
            }
        }
        count
    }
}

impl BfsSolution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] != '1' {
                    continue;
                }
                count += 1;
                grid[r][c] = '0';
                let mut queue = VecDeque::from([(r, c)]);
                while let Some((row, col)) = queue.pop_front() {
                    for (nr, nc) in neighbors(row, col, grid.len(), grid[0].len()) {
                        if grid[nr][nc] == '1' {
                            grid[nr][nc] = '0';
                            queue.push_back((nr, nc));
                        }
                    }
                }
            }
        }
        count
    }
}

impl UnionFindSolution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut uf = UnionFind::new(rows * cols);
        let mut count = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    count += 1;
                    if r + 1 < rows && grid[r + 1][c] == '1' && uf.union(r * cols + c, (r + 1) * cols + c) {
                        count -= 1;
                    }
                    if c + 1 < cols && grid[r][c + 1] == '1' && uf.union(r * cols + c, r * cols + c + 1) {
                        count -= 1;
                    }
                }
            }
        }
        count
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

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.rank[ra] < self.rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        if self.rank[ra] == self.rank[rb] {
            self.rank[ra] += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(grid: Vec<Vec<char>>, expected: i32) {
        assert_eq!(Solution::num_islands(grid.clone()), expected);
        assert_eq!(BfsSolution::num_islands(grid.clone()), expected);
        assert_eq!(UnionFindSolution::num_islands(grid), expected);
    }

    #[test]
    fn single_island() {
        assert_all_solutions(
            vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ],
            1,
        );
    }

    #[test]
    fn multiple_islands() {
        assert_all_solutions(
            vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ],
            3,
        );
    }
}
