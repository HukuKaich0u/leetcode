/*
この問題で鍛えること:
- アルゴリズム: LIS の O(n^2) DP と O(n log n) tails 法
- データ構造: tails 配列
- Rust での練習ポイント: `partition_point`、DP 配列更新、helper 化
- コード設計: 「実際の列」ではなく「長さごとの最良末尾」を持つ抽象化

問題の本質:
- 実際の増加部分列そのものを全部覚える必要はない
- 長さ `l` の増加部分列について、末尾が小さいほど将来つなげやすい
- だから「各長さの増加部分列のうち、末尾が最小のもの」だけを持てば十分

解法候補:
1. O(n^2) DP
   - 時間計算量: O(n^2)
   - 空間計算量: O(n)
   - 採用/非採用理由: 理解の入口として重要
2. tails + binary search
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 本命。抽象化が強い
3. helper で tails 更新を切り出す版
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - 採用/非採用理由: tails 更新の意味を関数名に乗せられる

採用解法の説明:
- `tails[len - 1]` を「長さ `len` の増加部分列の末尾の最小値」とする
- 新しい値 `value` ごとに、`tails` の中で `value` 以上になる最初の位置を探す
- そこを `value` で置き換えると、その長さの部分列の末尾をより小さくできる
- もし末尾まで到達したら、新しい長さの LIS を 1 つ伸ばせる

Rust観点の解説:
- `partition_point(|&x| x < value)` を使うと lower_bound を簡潔に書ける
- O(n^2) DP 版は `dp[i] = i で終わる LIS 長` として素直に実装できる
- helper 版では tails 更新責務を独立させると読みやすい

いいコード観点の解説:
- 本命実装では `tails` が何を意味するかを解説で固定しないと読者が迷う
- DP 版を比較対象に置くと、tails 法が「なぜ速いか」だけでなく「何を捨てているか」も見える
- helper 版は再利用性が高いが、tails の意味を理解した後に読むのがよい

落とし穴:
- `tails` を実際の LIS そのものだと誤解する
- 重複値で長さを伸ばしてしまう
- `partition_point` の比較を `<=` にしてしまい、strict increasing 条件を壊す
*/

struct Solution;
struct QuadraticDpSolution;
struct HelperTailsSolution;

impl QuadraticDpSolution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut dp = vec![1; nums.len()];
        let mut best = 1;

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            best = best.max(dp[i]);
        }

        best
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = Vec::new();

        for value in nums {
            let index = tails.partition_point(|&tail| tail < value);
            if index == tails.len() {
                tails.push(value);
            } else {
                tails[index] = value;
            }
        }

        tails.len() as i32
    }
}

impl HelperTailsSolution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tails = Vec::new();

        for value in nums {
            Self::update_tails(&mut tails, value);
        }

        tails.len() as i32
    }

    fn update_tails(tails: &mut Vec<i32>, value: i32) {
        let index = tails.partition_point(|&tail| tail < value);
        if index == tails.len() {
            tails.push(value);
        } else {
            tails[index] = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, expected: i32) {
        assert_eq!(QuadraticDpSolution::length_of_lis(nums.clone()), expected);
        assert_eq!(Solution::length_of_lis(nums.clone()), expected);
        assert_eq!(HelperTailsSolution::length_of_lis(nums), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![10, 9, 2, 5, 3, 7, 101, 18], 4);
    }

    #[test]
    fn duplicates_do_not_extend_lis() {
        assert_all_solutions(vec![7, 7, 7, 7, 7], 1);
    }

    #[test]
    fn decreasing_case() {
        assert_all_solutions(vec![5, 4, 3, 2, 1], 1);
    }

    #[test]
    fn mixed_case() {
        assert_all_solutions(vec![0, 1, 0, 3, 2, 3], 4);
    }
}
