/*
この問題で鍛えること:
- アルゴリズム: 頻度集計 + top-k
- データ構造: bucket, BinaryHeap, HashMap
- Rust での練習ポイント: `BinaryHeap`、bucket 配列、頻度 map
- コード設計: 全 sort を避ける理由を説明できるようにする

問題の本質:
- まず各値の頻度を数える必要がある
- その上で欲しいのは全順序ではなく上位 `k` 個だけ
- したがって「全部 sort する必要があるか」を考えるのが本体

解法候補:
1. 頻度一覧を作って sort
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 実装は単純だが余分に並べる
2. bucket sort
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 頻度の上界が `n` なので本命
3. size `k` の min-heap
   - 時間計算量: O(n log k)
   - 空間計算量: O(n + k)
   - 採用/非採用理由: `k` が小さいときの比較対象として有益

採用解法の説明:
- 値の頻度は最大でも `nums.len()` まで
- したがって `bucket[freq] = その頻度の値一覧` という配列を作れる
- 高頻度側から bucket を下れば、必要な `k` 個だけ拾える
- 全体 sort を避けられるのがこの問題のきれいな点

Rust観点の解説:
- `HashMap<i32, usize>` で頻度を集計する
- `vec![Vec::new(); nums.len() + 1]` で bucket を作る
- heap 版では `BinaryHeap<Reverse<(usize, i32)>>` を使うと min-heap 的に扱える

いいコード観点の解説:
- 本命実装は制約を活かしていて、なぜ O(n) になるかが読み取れる
- sort 版を置くと、「まず通す」実装と「構造を使う」実装の差が見える
- heap 版は top-k テンプレートとして別問題にも流用しやすい

落とし穴:
- 頻度を数えた後にそのまま全体 sort してしまい、制約を使い切らない
- bucket のサイズを `nums.len()` ではなく distinct 数にして壊す
- heap 版で size `k` を超えたときの pop を忘れる
*/

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Solution;
struct SortSolution;
struct HeapSolution;
struct RefinedSolution;

impl SortSolution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for value in nums {
            *count.entry(value).or_insert(0usize) += 1;
        }

        let mut pairs: Vec<(i32, usize)> = count.into_iter().collect();
        pairs.sort_unstable_by_key(|&(_, freq)| Reverse(freq));
        pairs
            .into_iter()
            .take(k as usize)
            .map(|(value, _)| value)
            .collect()
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for value in nums.iter().copied() {
            *count.entry(value).or_insert(0usize) += 1;
        }

        let mut buckets = vec![Vec::new(); nums.len() + 1];
        for (value, freq) in count {
            buckets[freq].push(value);
        }

        let mut result = Vec::new();
        for freq in (1..buckets.len()).rev() {
            for &value in &buckets[freq] {
                result.push(value);
                if result.len() == k as usize {
                    return result;
                }
            }
        }

        result
    }
}

impl HeapSolution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for value in nums {
            *count.entry(value).or_insert(0usize) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (value, freq) in count {
            heap.push(Reverse((freq, value)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.into_iter().map(|Reverse((_, value))| value).collect()
    }
}

impl RefinedSolution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let count = Self::build_count(nums);
        Self::collect_from_buckets(count, k)
    }

    fn build_count(nums: Vec<i32>) -> HashMap<i32, usize> {
        let mut count = HashMap::new();
        for value in nums {
            *count.entry(value).or_insert(0usize) += 1;
        }
        count
    }

    fn collect_from_buckets(count: HashMap<i32, usize>, k: i32) -> Vec<i32> {
        let max_freq = count.values().copied().max().unwrap_or(0);
        let mut buckets = vec![Vec::new(); max_freq + 1];
        for (value, freq) in count {
            buckets[freq].push(value);
        }

        let mut result = Vec::new();
        for freq in (1..buckets.len()).rev() {
            for &value in &buckets[freq] {
                result.push(value);
                if result.len() == k as usize {
                    return result;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums
    }

    fn assert_all_solutions(nums: Vec<i32>, k: i32, expected_options: Vec<Vec<i32>>) {
        let mut expected_options: Vec<Vec<i32>> =
            expected_options.into_iter().map(normalize).collect();
        expected_options.sort();

        for actual in [
            SortSolution::top_k_frequent(nums.clone(), k),
            Solution::top_k_frequent(nums.clone(), k),
            HeapSolution::top_k_frequent(nums.clone(), k),
            RefinedSolution::top_k_frequent(nums.clone(), k),
        ] {
            let actual = normalize(actual);
            assert!(expected_options.contains(&actual));
        }
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![1, 1, 1, 2, 2, 3], 2, vec![vec![1, 2]]);
    }

    #[test]
    fn single_result_case() {
        assert_all_solutions(vec![1], 1, vec![vec![1]]);
    }

    #[test]
    fn tied_counts_case() {
        assert_all_solutions(
            vec![4, 4, 1, 1, 2, 2],
            2,
            vec![vec![1, 2], vec![1, 4], vec![2, 4]],
        );
    }
}
