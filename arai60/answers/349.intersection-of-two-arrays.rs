/*
この問題で鍛えること:
- アルゴリズム: membership 判定の高速化
- データ構造: HashSet
- Rust での練習ポイント: `collect::<HashSet<_>>()`、集合演算、sort + two pointers
- コード設計: 二重ループから set 化へ抽象度を上げる感覚

問題の本質:
- 欲しいのは重複なしの共通要素だけ
- つまり、各値について「相手側に存在するか」と「結果にすでに入れたか」だけ分かればよい
- これは HashSet の要件とそのまま一致する

解法候補:
1. 二重ループで全部比較
   - 時間計算量: O(nm)
   - 空間計算量: O(k)
   - 採用/非採用理由: 単純だが membership 判定を毎回やり直している
2. HashSet
   - 時間計算量: 平均 O(n + m)
   - 空間計算量: O(n + m)
   - 採用/非採用理由: 要件に最も自然で、本命
3. sort + two pointers
   - 時間計算量: O(n log n + m log m)
   - 空間計算量: O(1) 追加少なめ
   - 採用/非採用理由: hash を使いたくない場合の比較対象として有益

採用解法の説明:
- 片方または両方を set 化すれば、存在判定は平均 O(1)
- 結果も set にしておけば、重複を自然に消せる
- そのため、集合としての交差をそのままコードに落とせばよい

Rust観点の解説:
- `nums.into_iter().collect::<HashSet<_>>()` で簡潔に set 化できる
- `intersection(&other)` は `&T` を返すので、`copied()` して値を取り出す
- sort 版では `Vec<i32>` を `sort_unstable()` して index を進める

いいコード観点の解説:
- 本命実装は「何をしたいか」がライブラリ呼び出しにそのまま乗る
- sort 版を並べると、hash を使う意味と、順序構造を使う意味の違いが見える
- 集合問題は返り値順が本質でないことが多いので、テスト側で順序非依存にするのがよい

落とし穴:
- 結果の重複を消し忘れる
- テストで順序まで固定してしまい、本質でない失敗を作る
- 二重ループ版で結果重複を避ける条件が複雑になる
*/

use std::collections::HashSet;

struct Solution;
struct BruteForceSolution;
struct SortedTwoPointersSolution;
struct RefinedSolution;

impl BruteForceSolution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for &value1 in &nums1 {
            for &value2 in &nums2 {
                if value1 == value2 && !result.contains(&value1) {
                    result.push(value1);
                }
            }
        }

        result
    }
}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();
        set1.intersection(&set2).copied().collect()
    }
}

impl SortedTwoPointersSolution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut left = 0usize;
        let mut right = 0usize;
        let mut result = Vec::new();

        while left < nums1.len() && right < nums2.len() {
            match nums1[left].cmp(&nums2[right]) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right += 1,
                std::cmp::Ordering::Equal => {
                    if result.last() != Some(&nums1[left]) {
                        result.push(nums1[left]);
                    }
                    left += 1;
                    right += 1;
                }
            }
        }

        result
    }
}

impl RefinedSolution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let smaller: HashSet<i32> = nums1.into_iter().collect();
        let larger: HashSet<i32> = nums2.into_iter().collect();
        Self::collect_intersection(&smaller, &larger)
    }

    fn collect_intersection(left: &HashSet<i32>, right: &HashSet<i32>) -> Vec<i32> {
        left.intersection(right).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums1: Vec<i32>, nums2: Vec<i32>, mut expected: Vec<i32>) {
        expected.sort_unstable();

        let mut brute_force = BruteForceSolution::intersection(nums1.clone(), nums2.clone());
        brute_force.sort_unstable();
        assert_eq!(brute_force, expected);

        let mut main = Solution::intersection(nums1.clone(), nums2.clone());
        main.sort_unstable();
        assert_eq!(main, expected);

        let mut sorted = SortedTwoPointersSolution::intersection(nums1.clone(), nums2.clone());
        sorted.sort_unstable();
        assert_eq!(sorted, expected);

        let mut refined = RefinedSolution::intersection(nums1, nums2);
        refined.sort_unstable();
        assert_eq!(refined, expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![1, 2, 2, 1], vec![2, 2], vec![2]);
    }

    #[test]
    fn no_overlap_case() {
        assert_all_solutions(vec![1, 3], vec![2, 4], vec![]);
    }

    #[test]
    fn multiple_results_case() {
        assert_all_solutions(vec![4, 9, 5], vec![9, 4, 9, 8, 4], vec![4, 9]);
    }
}
