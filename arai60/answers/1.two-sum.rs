/*
この問題で鍛えること:
- アルゴリズム: 線形走査を保ったまま探索を高速化する考え方
- データ構造: HashMap による逆引き
- Rust での練習ポイント: `HashMap::get`、`enumerate()`、`Option` 分岐
- コード設計: 素朴実装から、説明しやすい実装、少し整理した実装へ育てる感覚

問題の本質:
- 各要素 `nums[i]` に対して必要なのは、`target - nums[i]` がすでに出たかどうかだけ
- 未来の要素はまだ見ていないので、過去の要素だけを逆引きできれば十分
- つまり「全組を調べる問題」ではなく、「現在値の相方を即座に探す問題」に言い換えられる

解法候補:
1. 二重ループ
   - 考え方: 全ての組 `(i, j)` を調べる
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - データ構造: 特になし
   - 採用/非採用理由: 一番素直だが、改善余地が明確なので本命にはしない
2. HashMap で過去要素を逆引き
   - 考え方: `need = target - value` が過去にあるかを 1 回の走査で確認する
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - データ構造: `HashMap<i32, usize>`
   - 採用/非採用理由: 本質が最も見えやすく、面接でも実務でも説明しやすいので本命
3. sort + two pointers
   - 考え方: 値で並べて両端から寄せる
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - データ構造: ソート済み配列
   - 採用/非採用理由: index を失うので補助情報が必要になり、学習効率が下がる

採用解法の説明:
- 走査位置 `i` で必要なのは、`nums[i]` と足して `target` になる過去の値だけ
- そのため、`seen[value] = index` を保ちながら左から右へ走査すればよい
- 不変条件は「`seen` には現在より左側の値しか入っていない」こと
- この不変条件のおかげで、`seen` から見つかった index と現在 index をそのまま答えにできる
- 現在値を先に `seen` へ入れると同じ要素を 2 回使う危険があるので、探索してから登録する順序が重要

Rust観点の解説:
- `for (i, &value) in nums.iter().enumerate()` で、所有権を動かさずに index と値を同時に取れる
- `HashMap::get` は `Option<&usize>` を返すので、`if let Some(&j)` で参照を外して受ける
- index は `usize` だが、LeetCode の返り値は `Vec<i32>` なので最後に変換する

いいコード観点の解説:
- `need` という名前にして、式そのものではなく意味を変数名に乗せる
- 「探索してから登録する」順序がコード上でも上から自然に読める形にする
- 整理した実装では、`find_pair_indices` を切り出して「探索」と「返り値の整形」を分ける
- ただし、切り出しすぎるとかえって流れが見えにくいので、本命実装は 1 メソッドで保つ

落とし穴:
- 現在の値を先に map へ入れてしまい、同じ index を 2 回使う
- sort したあとに元 index を復元できなくなる
- `HashMap` の値として「最後の index」だけを持つ実装にしてもこの問題は解けるが、順序依存を理解せずに書くとバグりやすい
- index の型変換を後回しにしすぎて、途中で `usize` と `i32` を混ぜる
*/

use std::collections::HashMap;

struct Solution;
struct BruteForceSolution;
struct NormalizedIndexSolution;

impl BruteForceSolution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for left in 0..nums.len() {
            for right in left + 1..nums.len() {
                if nums[left] + nums[right] == target {
                    return vec![left as i32, right as i32];
                }
            }
        }
        unreachable!("problem guarantees exactly one answer");
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // seen[value] = その値を最初に見た index。
        // 現在値に必要な相方が過去にあるかだけを確認する。
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            let needed = target - value;
            if let Some(&pair_index) = seen.get(&needed) {
                return vec![pair_index as i32, index as i32];
            }

            // 現在値は、後続要素にとっての相方候補になる。
            seen.insert(value, index);
        }

        unreachable!("problem guarantees exactly one answer");
    }
}

impl NormalizedIndexSolution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (left, right) = Self::find_pair_indices(&nums, target)
            .expect("problem guarantees exactly one answer");
        vec![left as i32, right as i32]
    }

    fn find_pair_indices(nums: &[i32], target: i32) -> Option<(usize, usize)> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            let needed = target - value;
            if let Some(&pair_index) = seen.get(&needed) {
                return Some((pair_index, index));
            }
            seen.insert(value, index);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(nums: Vec<i32>, target: i32, expected: Vec<i32>) {
        assert_eq!(BruteForceSolution::two_sum(nums.clone(), target), expected);
        assert_eq!(Solution::two_sum(nums.clone(), target), expected);
        assert_eq!(NormalizedIndexSolution::two_sum(nums, target), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![2, 7, 11, 15], 9, vec![0, 1]);
    }

    #[test]
    fn duplicate_values_still_choose_distinct_indices() {
        assert_all_solutions(vec![3, 3], 6, vec![0, 1]);
    }

    #[test]
    fn negative_values_are_supported() {
        assert_all_solutions(vec![-1, -2, -3, -4, -5], -8, vec![2, 4]);
    }
}
