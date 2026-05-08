/*
この問題で鍛えること:
- アルゴリズム: binary search による境界探索
- データ構造: 追加構造なしの半区間絞り込み
- Rust での練習ポイント: `[left, right]` 型の不変条件、境界更新 helper
- コード設計: 線形探索から「最小値が左か右か」を判定する見方への移行

問題の本質:
- 回転済み昇順配列では、どこかで大小関係が崩れている
- 最小値そのものを直接探すより、「`mid` が最小値の左側か右側か」を判定する方が簡潔
- `nums[right]` を基準にすると、`nums[mid] > nums[right]` なら最小値は右半分にあると分かる

解法候補:
1. 線形探索
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 素朴だが二分探索の学びがない
2. right 端基準の binary search
   - 時間計算量: O(log n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 判定が簡潔で、本命
3. helper へ境界判定を切り出す版
   - 時間計算量: O(log n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 何を比較しているかが明示しやすい

採用解法の説明:
- 区間 `[left, right]` に最小値があると保つ
- `mid` を見て、`nums[mid] > nums[right]` なら右側に回転点があるので `left = mid + 1`
- そうでなければ `mid` 自身が最小候補なので `right = mid`
- `left == right` になった位置が最小値

Rust観点の解説:
- この問題は右端を含む判定を使うので、`[left, right]` 型で書くのが自然
- `while left < right` で回すと、終端条件が明快
- helper 版では「右側へ寄せる条件」を関数名に出せる

いいコード観点の解説:
- 本命実装では、比較式 `nums[mid] > nums[right]` が何を意味するかをコメントで固定する
- 線形探索版を比較対象に置くと、なぜ `O(log n)` へ持ち込めるかが見えやすい
- helper 版は境界更新の意味を分離できるが、本命ほど一直線ではない

落とし穴:
- `right = mid - 1` にしてしまい、`mid` 自身が最小値候補であることを落とす
- `nums[mid] < nums[left]` のような別条件で書いて、回転なしケースを壊す
- `[left, right)` と `[left, right]` を混ぜる
*/

struct Solution;
struct LinearSolution;
struct HelperBoundarySolution;

impl LinearSolution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        *nums.iter().min().unwrap()
    }
}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0usize, nums.len() - 1);

        while left < right {
            let mid = left + (right - left) / 2;

            // `mid` が右端より大きければ、最小値は回転点のある右半分にある。
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left]
    }
}

impl HelperBoundarySolution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if Self::minimum_is_on_right(&nums, mid, right) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left]
    }

    fn minimum_is_on_right(nums: &[i32], mid: usize, right: usize) -> bool {
        nums[mid] > nums[right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, expected: i32) {
        assert_eq!(LinearSolution::find_min(nums.clone()), expected);
        assert_eq!(Solution::find_min(nums.clone()), expected);
        assert_eq!(HelperBoundarySolution::find_min(nums), expected);
    }

    #[test]
    fn rotated_array_case() {
        assert_all_solutions(vec![3, 4, 5, 1, 2], 1);
    }

    #[test]
    fn already_sorted_array_case() {
        assert_all_solutions(vec![1, 2, 3, 4], 1);
    }

    #[test]
    fn single_element_case() {
        assert_all_solutions(vec![7], 7);
    }

    #[test]
    fn small_rotation_case() {
        assert_all_solutions(vec![2, 1], 1);
    }
}
