/*
この問題で鍛えること:
- アルゴリズム: rotated array 上の binary search
- データ構造: 追加構造なしの区間絞り込み
- Rust での練習ポイント: 整列している半分の判定、`i32` / `usize` 境界変換、helper 化
- コード設計: 線形探索から「毎回捨ててよい半分」を見つける思考

問題の本質:
- 配列全体は回転で壊れているが、`mid` を境に見れば少なくとも片側は整列している
- その整列している側に `target` が入るかどうかで、捨ててよい半分を決められる
- つまり「回転配列の探索」ではなく、「整列している半分を毎回特定する境界探索」

解法候補:
1. 線形探索
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 素朴だが、二分探索の学習価値が無い
2. 片側整列判定つき binary search
   - 時間計算量: O(log n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命。1 回の探索で済む
3. helper で整列側判定を切り出す版
   - 時間計算量: O(log n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 判定責務を分けて読むには有効

採用解法の説明:
- `mid` の値 `value` を取る
- `nums[left] <= value` なら左半分 `[left, mid]` は整列している
- このとき `target` がその範囲内なら右半分を捨て、そうでなければ左半分を捨てる
- 逆に左半分が整列していなければ、右半分 `[mid, right]` が整列しているので同じ考え方で絞る
- 毎回必ず半分を捨てられるので O(log n)

Rust観点の解説:
- 本命実装では `left` / `right` を `i32` で持ち、`mid` 計算後だけ `usize` へ変換する
- helper 版では「左半分が整列しているか」「target が範囲内か」を関数名に出すと読みやすい
- 添字アクセスは `left as usize` のように変換するが、負数にならない不変条件をコード全体で保つ

いいコード観点の解説:
- 分岐が多い問題なので、「何を判定しているか」をコメントか helper 名で固定するのが大事
- 本命実装は 1 メソッドで全体像が追える形にする
- helper 版はロジックを分割するが、分けすぎて流れを見えなくしないバランスが必要

落とし穴:
- 左右どちらが整列しているかを誤判定する
- `target` が整列区間内にある条件の不等号を 1 つ間違える
- 回転していない配列を特別扱いしすぎて、かえって分岐を壊す
*/

struct Solution;
struct LinearSolution;
struct HelperSearchSolution;

impl LinearSolution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for (index, value) in nums.into_iter().enumerate() {
            if value == target {
                return index as i32;
            }
        }
        -1
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0i32, nums.len() as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let value = nums[mid as usize];
            if value == target {
                return mid;
            }

            if nums[left as usize] <= value {
                if nums[left as usize] <= target && target < value {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else if value < target && target <= nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }
}

impl HelperSearchSolution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0i32, nums.len() as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let value = nums[mid as usize];
            if value == target {
                return mid;
            }

            if Self::left_half_is_sorted(&nums, left, mid) {
                if Self::target_in_left_half(&nums, left, mid, target) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else if Self::target_in_right_half(&nums, mid, right, target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        -1
    }

    fn left_half_is_sorted(nums: &[i32], left: i32, mid: i32) -> bool {
        nums[left as usize] <= nums[mid as usize]
    }

    fn target_in_left_half(nums: &[i32], left: i32, mid: i32, target: i32) -> bool {
        nums[left as usize] <= target && target < nums[mid as usize]
    }

    fn target_in_right_half(nums: &[i32], mid: i32, right: i32, target: i32) -> bool {
        nums[mid as usize] < target && target <= nums[right as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, target: i32, expected: i32) {
        assert_eq!(LinearSolution::search(nums.clone(), target), expected);
        assert_eq!(Solution::search(nums.clone(), target), expected);
        assert_eq!(HelperSearchSolution::search(nums, target), expected);
    }

    #[test]
    fn finds_target_in_rotated_array() {
        assert_all_solutions(vec![4, 5, 6, 7, 0, 1, 2], 0, 4);
    }

    #[test]
    fn returns_negative_one_when_missing() {
        assert_all_solutions(vec![4, 5, 6, 7, 0, 1, 2], 3, -1);
    }

    #[test]
    fn handles_not_rotated_case() {
        assert_all_solutions(vec![1, 2, 3, 4, 5], 4, 3);
    }

    #[test]
    fn handles_small_array_case() {
        assert_all_solutions(vec![5, 1, 3], 5, 0);
    }
}
