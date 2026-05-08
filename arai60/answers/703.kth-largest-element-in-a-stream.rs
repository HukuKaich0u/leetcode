/*
この問題で鍛えること:
- アルゴリズム: データストリームの top-k 維持
- データ構造: min-heap / sorted vector
- Rust での練習ポイント: `BinaryHeap<Reverse<_>>`, stateful struct, 小さい API 設計
- コード設計: 毎回 sort し直すのではなく、必要な情報だけを持ち続ける発想

問題の本質:
- すべての値を毎回並べ替える必要はない
- kth largest を知りたいなら、上位 k 個だけ保持すれば十分
- その集合の最小値がちょうど kth largest になる

解法候補:
1. 追加ごとに全体 sort
   - 1 回の `add` あたり O(n log n)
   - 採用/非採用理由: 実装は簡単だが無駄が大きい
2. 上位 k 個だけを持つ min-heap
   - 1 回の `add` あたり O(log k)
   - 採用/非採用理由: 本命
3. sorted vector を常に保つ
   - 1 回の `add` あたり O(n)
   - 採用/非採用理由: heap との比較用に有益

採用解法の説明:
- heap には「これまで見た値のうち大きい方から k 個」だけを残す
- 新しい値を入れたあと、サイズが k を超えたら最小値を 1 個捨てる
- すると heap の先頭が常に kth largest

Rust観点の解説:
- `BinaryHeap` を min-heap として使うため `Reverse<i32>` を使う
- 設計問題なので、状態を `struct` のフィールドに素直に持つ
- sorted vector 版では `partition_point` が「挿入位置探索」にちょうどよい

いいコード観点の解説:
- この問題はアルゴリズムより「永続状態をどう設計するか」の比重が高い
- `add` が短くなるほど、持つべき状態が適切である証拠になる
- 別解を並べると、heap が top-k 維持に特化したデータ構造だと実感しやすい

落とし穴:
- 全要素を保持したまま毎回 sort してしまう
- max-heap のまま使って kth largest ではなく largest を返してしまう
- `k` 個未満の初期状態で返り値の意味を崩す
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

struct KthLargestNaive {
    k: usize,
    values: Vec<i32>,
}

struct KthLargestSorted {
    k: usize,
    values: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut this = Self {
            k: k as usize,
            heap: BinaryHeap::new(),
        };
        for value in nums {
            this.add(value);
        }
        this
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

impl KthLargestNaive {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            k: k as usize,
            values: nums,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.values.push(val);
        self.values.sort_unstable_by(|a, b| b.cmp(a));
        self.values[self.k - 1]
    }
}

impl KthLargestSorted {
    fn new(k: i32, mut nums: Vec<i32>) -> Self {
        nums.sort_unstable();
        Self {
            k: k as usize,
            values: nums,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        let index = self.values.partition_point(|&value| value <= val);
        self.values.insert(index, val);
        self.values[self.values.len() - self.k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_sequence() {
        let mut naive = KthLargestNaive::new(3, vec![4, 5, 8, 2]);
        let mut main = KthLargest::new(3, vec![4, 5, 8, 2]);
        let mut sorted = KthLargestSorted::new(3, vec![4, 5, 8, 2]);

        let expected = [4, 5, 5, 8, 8];
        let inputs = [3, 5, 10, 9, 4];

        for (&input, &answer) in inputs.iter().zip(expected.iter()) {
            assert_eq!(naive.add(input), answer);
            assert_eq!(main.add(input), answer);
            assert_eq!(sorted.add(input), answer);
        }
    }

    #[test]
    fn handles_single_slot_heap() {
        let mut naive = KthLargestNaive::new(1, vec![]);
        let mut main = KthLargest::new(1, vec![]);
        let mut sorted = KthLargestSorted::new(1, vec![]);

        for &(input, answer) in [(-3, -3), (-2, -2), (-4, -2), (0, 0), (4, 4)].iter() {
            assert_eq!(naive.add(input), answer);
            assert_eq!(main.add(input), answer);
            assert_eq!(sorted.add(input), answer);
        }
    }
}
