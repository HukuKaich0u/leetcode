/*
この問題で鍛えること:
- アルゴリズム: DFS/BFS, Union Find
- データ構造: adjacency list, disjoint set union
- Rust での練習ポイント: 隣接リスト構築、path compression
- コード設計: 「成分数をどう減らしていくか」の見方

問題の本質:
- 最初は `n` 個の頂点がそれぞれ独立成分
- 辺を読むたび、別成分同士をつなぐなら成分数は 1 減る
- DFS で数えても、Union Find で併合しても本質は同じ

解法候補:
1. DFS
2. BFS
3. Union Find
   - 採用/非採用理由: 本命。併合の意味が直接見える

採用解法の説明:
- parent 配列を持ち、各頂点の代表元を管理する
- 辺 `(u, v)` で `find(u) != find(v)` なら `union` して成分数を 1 減らす
- 最後に残った成分数が答え

Rust観点の解説:
- `Vec<Vec<usize>>` の隣接リストは DFS/BFS 別解で自然
- Union Find は `find` を再帰で書くと短い
- `usize` へ先に変換しておくと index が素直

いいコード観点の解説:
- Union Find は「何を state として持つか」が答えそのもの
- DFS/BFS を併記すると、同じ連結性問題でも表現方法が違うと分かる
- 成分数を減らす設計に気付けると、この型の問題が広く解ける

落とし穴:
- Union 済みの辺でも毎回 count を減らしてしまう
- 無向辺なのに片方向だけ adjacency へ入れる
- visited を忘れて DFS/BFS がループする
*/

use std::collections::VecDeque;

struct Solution;
struct DfsSolution;
struct BfsSolution;

impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut uf = UnionFind::new(n);
        let mut count = n as i32;

        for edge in edges {
            if uf.union(edge[0] as usize, edge[1] as usize) {
                count -= 1;
            }
        }

        count
    }
}

impl DfsSolution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let graph = build_graph(n as usize, &edges);
        let mut visited = vec![false; n as usize];
        let mut count = 0;

        for start in 0..n as usize {
            if visited[start] {
                continue;
            }
            count += 1;
            let mut stack = vec![start];
            while let Some(node) = stack.pop() {
                if visited[node] {
                    continue;
                }
                visited[node] = true;
                for &next in &graph[node] {
                    if !visited[next] {
                        stack.push(next);
                    }
                }
            }
        }

        count
    }
}

impl BfsSolution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let graph = build_graph(n as usize, &edges);
        let mut visited = vec![false; n as usize];
        let mut count = 0;

        for start in 0..n as usize {
            if visited[start] {
                continue;
            }
            count += 1;
            let mut queue = VecDeque::from([start]);
            visited[start] = true;
            while let Some(node) = queue.pop_front() {
                for &next in &graph[node] {
                    if !visited[next] {
                        visited[next] = true;
                        queue.push_back(next);
                    }
                }
            }
        }

        count
    }
}

fn build_graph(n: usize, edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
    let mut graph = vec![Vec::new(); n];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        graph[a].push(b);
        graph[b].push(a);
    }
    graph
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

    fn assert_all_solutions(n: i32, edges: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(Solution::count_components(n, edges.clone()), expected);
        assert_eq!(DfsSolution::count_components(n, edges.clone()), expected);
        assert_eq!(BfsSolution::count_components(n, edges), expected);
    }

    #[test]
    fn two_components() {
        assert_all_solutions(5, vec![vec![0, 1], vec![1, 2], vec![3, 4]], 2);
    }

    #[test]
    fn single_component() {
        assert_all_solutions(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]], 1);
    }
}
