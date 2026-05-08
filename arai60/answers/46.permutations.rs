/*
この問題で鍛えること:
- アルゴリズム: permutations generation
- データ構造: used 配列 / in-place swap
- Rust での練習ポイント: push/pop と swap、再帰での可変借用
- コード設計: 「残り集合」をどう表現するか

問題の本質:
- 各位置に未使用の要素を 1 つずつ置いていく探索
- 組み合わせと違って順序が意味を持つので、`start` だけでは状態が足りない
- 未使用要素の管理方法が実装差になる

解法候補:
1. 残り要素 vector を毎回作り直す
   - 採用/非採用理由: わかりやすいがコピーが多い
2. used 配列を持つ backtracking
   - 採用/非採用理由: 本命
3. swap による in-place 生成
   - 採用/非採用理由: 配列操作の別解として強い

採用解法の説明:
- `path.len()` が現在埋めた位置数
- 各ステップで `used[i] == false` の要素を 1 つ選び、path へ積む
- 使ったら `used[i] = true`、戻るときに `false` へ戻す

Rust観点の解説:
- `used` と `path` を可変参照で回すと、余計な clone を減らせる
- swap 版は `nums.swap(first, i)` を前後で戻すのがポイント
- 順列問題では「戻す処理」が correctness の核になる

いいコード観点の解説:
- 本命実装は状態が `used` と `path` に明示され、探索木が読み取りやすい
- swap 版はメモリ効率がよいが、状態変化が強いため少し読み手を選ぶ
- 残り配列を毎回作る素朴版も、状態設計の比較には役立つ

落とし穴:
- 戻りがけに `used[i] = false` を忘れる
- path clone の位置を誤る
- swap を戻し忘れて後続枝を壊す
*/

struct Solution;
struct NaiveSolution;
struct SwapSolution;

impl NaiveSolution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(remaining: Vec<i32>, path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
            if remaining.is_empty() {
                out.push(path.clone());
                return;
            }
            for i in 0..remaining.len() {
                let mut next_remaining = remaining.clone();
                let value = next_remaining.remove(i);
                path.push(value);
                dfs(next_remaining, path, out);
                path.pop();
            }
        }

        let mut out = Vec::new();
        let mut path = Vec::new();
        dfs(nums, &mut path, &mut out);
        out
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &[i32], used: &mut [bool], path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                out.push(path.clone());
                return;
            }

            for i in 0..nums.len() {
                if used[i] {
                    continue;
                }
                used[i] = true;
                path.push(nums[i]);
                dfs(nums, used, path, out);
                path.pop();
                used[i] = false;
            }
        }

        let mut used = vec![false; nums.len()];
        let mut out = Vec::new();
        let mut path = Vec::new();
        dfs(&nums, &mut used, &mut path, &mut out);
        out
    }
}

impl SwapSolution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = Vec::new();
        Self::dfs(0, &mut nums, &mut out);
        out
    }

    fn dfs(first: usize, nums: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if first == nums.len() {
            out.push(nums.clone());
            return;
        }
        for i in first..nums.len() {
            nums.swap(first, i);
            Self::dfs(first + 1, nums, out);
            nums.swap(first, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut values: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        values.sort();
        values
    }

    fn assert_all_solutions(nums: Vec<i32>, expected: Vec<Vec<i32>>) {
        let expected = normalize(expected);
        assert_eq!(normalize(NaiveSolution::permute(nums.clone())), expected);
        assert_eq!(normalize(Solution::permute(nums.clone())), expected);
        assert_eq!(normalize(SwapSolution::permute(nums)), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(
            vec![1, 2, 3],
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
        );
    }

    #[test]
    fn single_element_case() {
        assert_all_solutions(vec![1], vec![vec![1]]);
    }
}
