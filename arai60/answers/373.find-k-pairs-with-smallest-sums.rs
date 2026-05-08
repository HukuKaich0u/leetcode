/*
この問題で鍛えること:
- アルゴリズム: best-first search, k-way merge
- データ構造: min-heap
- Rust での練習ポイント: `BinaryHeap<Reverse<_>>`, index 管理
- コード設計: 「全部作る」より「必要な順に広げる」発想

問題の本質:
- `nums1[i] + nums2[j]` を行列とみると、各行・各列で単調増加する
- 最小 pair から順に取り出し、その隣だけ広げればよい
- つまり各行の先頭を持つ k-way merge と同じ構造

解法候補:
1. 全 pair 列挙して sort
   - 採用/非採用理由: O(mn log mn) で重い
2. heap に各行の先頭だけを載せる
   - 採用/非採用理由: 本命
3. 行列 BFS 的に訪問管理しながら広げる
   - 採用/非採用理由: 別解だが本命より実装が重い

採用解法の説明:
- まず各行 `i` の `(i, 0)` を heap に入れる
- 最小和 pair を 1 つ取り出したら、その行の次 `(i, j+1)` だけを入れる
- これで各行から「未出力の最小 pair」だけが heap に残り続ける

Rust観点の解説:
- min-heap 化のため `Reverse<(sum, i, j)>` を使う
- `nums1.len().min(k as usize)` だけ初期投入すれば十分
- index を tuple で持つと、値自体は出力時まで再計算不要

いいコード観点の解説:
- 行列全体を materialize しないのが設計上の本質
- best-first search の状態を最小限に絞ると `add` 条件も自然に短くなる
- 類題の「k 個だけ欲しい」問題全般へつながる考え方

落とし穴:
- 全 pair を本当に作ってメモリを使い切る
- 行の次ではなく列の次も無条件に入れて重複管理が必要になる
- 初期 heap に全行を入れすぎる
*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct Solution;
struct NaiveSolution;
struct MatrixBfsSolution;

impl NaiveSolution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut pairs = Vec::new();
        for &a in &nums1 {
            for &b in &nums2 {
                pairs.push(vec![a, b]);
            }
        }
        pairs.sort_unstable_by_key(|pair| pair[0] + pair[1]);
        pairs.into_iter().take(k as usize).collect()
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if nums1.is_empty() || nums2.is_empty() || k == 0 {
            return vec![];
        }

        let mut heap = BinaryHeap::new();
        for i in 0..nums1.len().min(k as usize) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0usize)));
        }

        let mut out = Vec::new();
        while let Some(Reverse((_, i, j))) = heap.pop() {
            out.push(vec![nums1[i], nums2[j]]);
            if out.len() == k as usize {
                break;
            }
            if j + 1 < nums2.len() {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        out
    }
}

impl MatrixBfsSolution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        if nums1.is_empty() || nums2.is_empty() || k == 0 {
            return vec![];
        }

        let mut heap = BinaryHeap::new();
        let mut seen = HashSet::new();
        heap.push(Reverse((nums1[0] + nums2[0], 0usize, 0usize)));
        seen.insert((0usize, 0usize));

        let mut out = Vec::new();
        while let Some(Reverse((_, i, j))) = heap.pop() {
            out.push(vec![nums1[i], nums2[j]]);
            if out.len() == k as usize {
                break;
            }
            if i + 1 < nums1.len() && seen.insert((i + 1, j)) {
                heap.push(Reverse((nums1[i + 1] + nums2[j], i + 1, j)));
            }
            if j + 1 < nums2.len() && seen.insert((i, j + 1)) {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums1: Vec<i32>, nums2: Vec<i32>, k: i32, expected: Vec<Vec<i32>>) {
        assert_eq!(NaiveSolution::k_smallest_pairs(nums1.clone(), nums2.clone(), k), expected);
        assert_eq!(Solution::k_smallest_pairs(nums1.clone(), nums2.clone(), k), expected);
        assert_eq!(MatrixBfsSolution::k_smallest_pairs(nums1, nums2, k), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![1, 7, 11], vec![2, 4, 6], 3, vec![vec![1, 2], vec![1, 4], vec![1, 6]]);
    }

    #[test]
    fn duplicate_values_case() {
        assert_all_solutions(vec![1, 1, 2], vec![1, 2, 3], 2, vec![vec![1, 1], vec![1, 1]]);
    }
}
