/*
この問題で鍛えること:
- アルゴリズム: sliding window が成立する条件の見抜き方
- データ構造: prefix sum と二分探索の組み合わせ
- Rust での練習ポイント: `usize` の長さ管理、`partition_point`、範囲走査
- コード設計: 全探索から本命解法へ絞り込み、必要なら window 更新だけ切り出す感覚

問題の本質:
- すべての要素が正なので、区間を右に伸ばすと和は増え、左を縮めると和は減る
- この単調性があるから、条件を満たしたあとに左を縮める操作が安全にできる
- 問題は「最短区間を全探索する」のではなく、「条件を満たした window をどこまで縮められるか」を管理する問題

解法候補:
1. 全区間を調べる
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 問題の仕様確認にはよいが、正の数という制約を使えていない
2. sliding window
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 正の数という条件を最も素直に使えて、本命
3. prefix sum + binary search
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 汎用性はあるが、この問題では window の方が簡潔

採用解法の説明:
- `right` を 1 つ進めるたびに区間和を増やす
- もし和が `target` 以上になったら、その区間は条件を満たしている
- その状態では、左端を削るほど長さは良くなるので、条件を壊す直前まで縮めるのが最善
- 不変条件は「`[left, right]` は現在考えている window で、和は `sum` に正しく対応している」こと
- 正の数しかないため、左を削ったあとに和が増えることはなく、縮小の判断が単調になる

Rust観点の解説:
- 長さは `usize` で持ち、最後に LeetCode 用に `i32` へ変換する
- `usize::MAX` を未更新の番兵値に使うと、`Option<usize>` を使わずに済む
- prefix sum 版では `partition_point` を使うと、「初めて条件を満たす位置」を明示的に探せる

いいコード観点の解説:
- sliding window は 1 メソッドで十分読みやすいので、本命は切り出しすぎない
- 一方、整理版では `shrink_while_possible` を補助関数にして、更新責務を分ける価値がある
- 何が単調で、どの条件なら window を縮めてよいかが読んだ瞬間に分かる形を優先する

落とし穴:
- 負数が混ざる問題と同じ感覚で sliding window を使ってしまう
- `sum >= target` の間に縮め続けるべきところを 1 回しか縮めない
- 解がないときに `usize::MAX` をそのまま返す
- prefix sum 版で「初めて必要和を超える位置」を取り損ねる
*/

struct Solution;
struct BruteForceSolution;
struct PrefixBinarySearchSolution;
struct RefinedWindowSolution;

impl BruteForceSolution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut best = usize::MAX;

        for left in 0..nums.len() {
            let mut sum = 0;
            for right in left..nums.len() {
                sum += nums[right];
                if sum >= target {
                    best = best.min(right - left + 1);
                    break;
                }
            }
        }

        if best == usize::MAX {
            0
        } else {
            best as i32
        }
    }
}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut sum = 0;
        let mut best = usize::MAX;

        for right in 0..nums.len() {
            sum += nums[right];

            // 正の数しかないので、条件を満たす間は左を縮めるほど得をする。
            while sum >= target {
                best = best.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }

        if best == usize::MAX {
            0
        } else {
            best as i32
        }
    }
}

impl PrefixBinarySearchSolution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];
        for (index, &value) in nums.iter().enumerate() {
            prefix[index + 1] = prefix[index] + value;
        }

        let mut best = usize::MAX;

        for left in 0..nums.len() {
            let required = prefix[left] + target;
            let right = prefix.partition_point(|&sum| sum < required);
            if right < prefix.len() {
                best = best.min(right - left);
            }
        }

        if best == usize::MAX {
            0
        } else {
            best as i32
        }
    }
}

impl RefinedWindowSolution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut sum = 0;
        let mut best = usize::MAX;

        for right in 0..nums.len() {
            sum += nums[right];
            Self::shrink_while_possible(target, &nums, &mut left, &mut sum, &mut best, right);
        }

        if best == usize::MAX {
            0
        } else {
            best as i32
        }
    }

    fn shrink_while_possible(
        target: i32,
        nums: &[i32],
        left: &mut usize,
        sum: &mut i32,
        best: &mut usize,
        right: usize,
    ) {
        while *sum >= target {
            *best = (*best).min(right - *left + 1);
            *sum -= nums[*left];
            *left += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(target: i32, nums: Vec<i32>, expected: i32) {
        assert_eq!(BruteForceSolution::min_sub_array_len(target, nums.clone()), expected);
        assert_eq!(Solution::min_sub_array_len(target, nums.clone()), expected);
        assert_eq!(
            PrefixBinarySearchSolution::min_sub_array_len(target, nums.clone()),
            expected
        );
        assert_eq!(RefinedWindowSolution::min_sub_array_len(target, nums), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(7, vec![2, 3, 1, 2, 4, 3], 2);
    }

    #[test]
    fn returns_zero_when_no_answer_exists() {
        assert_all_solutions(100, vec![1, 2, 3], 0);
    }

    #[test]
    fn handles_single_element_answer() {
        assert_all_solutions(4, vec![1, 4, 4], 1);
    }

    #[test]
    fn shrinks_window_multiple_times() {
        assert_all_solutions(11, vec![1, 2, 3, 4, 5], 3);
    }
}
