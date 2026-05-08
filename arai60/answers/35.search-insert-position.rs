/*
この問題で鍛えること:
- アルゴリズム: binary search と lower_bound
- データ構造: 追加構造なしの境界探索
- Rust での練習ポイント: 半開区間 `[left, right)`、`usize` 境界、helper 化
- コード設計: 線形探索から「最初に条件を満たす位置」へ見方を切り替える感覚

問題の本質:
- 欲しいのは「target が存在する位置」だけではなく、「無ければどこへ入るか」
- これは両方とも `nums[i] >= target` を初めて満たす位置、つまり lower_bound に統一できる
- したがって、この問題は通常の等値探索というより境界探索の問題

解法候補:
1. 線形探索
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 素朴だが binary search の練習にならない
2. lower_bound の binary search
   - 時間計算量: O(log n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命。存在位置と挿入位置を同じロジックで扱える
3. helper に切り出した lower_bound
   - 時間計算量: O(log n)
   - 空間計算量: O(1)
   - 採用/非採用理由: lower_bound を他問題へ使い回せる形に整理できる

採用解法の説明:
- 半開区間 `[left, right)` に答え候補があると考える
- `mid` で `nums[mid] < target` なら、答えは `mid + 1` 以右
- そうでなければ、`mid` 自身も答え候補なので右端を `mid` へ寄せる
- 最終的に `left == right` になった位置が lower_bound

Rust観点の解説:
- `right = nums.len()` として半開区間を作ると、末尾挿入ケースを特別扱いせずに済む
- `left + (right - left) / 2` と書くと、境界更新の意図が明確
- helper 版では `&[i32]` を受けて、検索ロジックだけを独立させられる

いいコード観点の解説:
- 「何を探しているか」を `lower_bound` と名前で固定すると、比較式の意味がぶれにくい
- 本命実装は 1 メソッドで十分だが、helper 版は他の二分探索問題への横展開に向く
- 線形探索版を比較で置くと、なぜ `O(log n)` にする価値があるかが見える

落とし穴:
- 存在する値を探す usual binary search のつもりで書き、挿入位置ケースを崩す
- `[left, right]` と `[left, right)` を混ぜる
- `nums[mid] <= target` にして right 更新を壊す
*/

struct Solution;
struct LinearSolution;
struct HelperLowerBoundSolution;

impl LinearSolution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        for (index, value) in nums.into_iter().enumerate() {
            if value >= target {
                return index as i32;
            }
        }
        len as i32
    }
}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0usize, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}

impl HelperLowerBoundSolution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::lower_bound(&nums, target) as i32
    }

    fn lower_bound(nums: &[i32], target: i32) -> usize {
        let (mut left, mut right) = (0usize, nums.len());

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, target: i32, expected: i32) {
        assert_eq!(LinearSolution::search_insert(nums.clone(), target), expected);
        assert_eq!(Solution::search_insert(nums.clone(), target), expected);
        assert_eq!(HelperLowerBoundSolution::search_insert(nums, target), expected);
    }

    #[test]
    fn existing_value_case() {
        assert_all_solutions(vec![1, 3, 5, 6], 5, 2);
    }

    #[test]
    fn insert_at_front_case() {
        assert_all_solutions(vec![1, 3, 5, 6], 0, 0);
    }

    #[test]
    fn insert_at_end_case() {
        assert_all_solutions(vec![1, 3, 5, 6], 7, 4);
    }
}
