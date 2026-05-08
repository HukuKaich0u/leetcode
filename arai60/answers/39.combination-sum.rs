/*
この問題で鍛えること:
- アルゴリズム: backtracking, branching control
- データ構造: 可変配列
- Rust での練習ポイント: sort, `Vec` の push/pop, 再帰引数の設計
- コード設計: 順序違い重複を状態制約で防ぐ方法

問題の本質:
- 候補を何回でも使えるが、順序違いの重複解は不要
- したがって「次に選んでよい候補の開始位置」を状態に持つ必要がある
- さらに候補を sort しておけば、`candidate > remain` で枝刈りできる

解法候補:
1. 順序ありで全列挙して正規化
   - 採用/非採用理由: 重複管理が重く、不自然
2. `start` index を持つ backtracking
   - 採用/非採用理由: 本命
3. include / skip の 2 分岐再帰
   - 採用/非採用理由: 選択木の見え方が明確

採用解法の説明:
- `start` より前の候補は使わないので、順序違い重複が出ない
- 候補 `candidates[i]` を選んだら、同じ要素を再利用できるので次も `i` から始める
- `remain == 0` で 1 解完成、`candidate > remain` でそれ以降は全部打ち切れる

Rust観点の解説:
- `candidates.sort_unstable()` で枝刈り条件を作る
- `path` は 1 本を使い回して push/pop する
- 別解の include/skip 版は状態が `(index, remain)` として見えやすい

いいコード観点の解説:
- 何を禁止して重複を消すのかが、`start` 引数 1 つで表現できている
- 列挙問題では「状態設計 = 重複管理」であることがよく分かる
- 枝刈り条件を sort と結び付けて説明できると強い

落とし穴:
- 再利用可能なのに `i + 1` へ進めてしまう
- 順序違いを重複解として大量生成する
- sort せずに `break` して正解を落とす
*/

struct Solution;
struct NaiveSolution;
struct HelperSolution;

impl NaiveSolution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            remain: i32,
            candidates: &[i32],
            path: &mut Vec<i32>,
            out: &mut Vec<Vec<i32>>,
        ) {
            if remain == 0 {
                let mut combination = path.clone();
                combination.sort_unstable();
                if !out.contains(&combination) {
                    out.push(combination);
                }
                return;
            }
            if remain < 0 {
                return;
            }
            for &candidate in candidates {
                path.push(candidate);
                dfs(remain - candidate, candidates, path, out);
                path.pop();
            }
        }

        let mut out = Vec::new();
        let mut path = Vec::new();
        dfs(target, &candidates, &mut path, &mut out);
        out
    }
}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            start: usize,
            remain: i32,
            candidates: &[i32],
            path: &mut Vec<i32>,
            out: &mut Vec<Vec<i32>>,
        ) {
            if remain == 0 {
                out.push(path.clone());
                return;
            }

            for i in start..candidates.len() {
                if candidates[i] > remain {
                    break;
                }
                path.push(candidates[i]);
                dfs(i, remain - candidates[i], candidates, path, out);
                path.pop();
            }
        }

        candidates.sort_unstable();
        let mut out = Vec::new();
        let mut path = Vec::new();
        dfs(0, target, &candidates, &mut path, &mut out);
        out
    }
}

impl HelperSolution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut out = Vec::new();
        let mut path = Vec::new();
        Self::dfs(0, target, &candidates, &mut path, &mut out);
        out
    }

    fn dfs(index: usize, remain: i32, candidates: &[i32], path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if remain == 0 {
            out.push(path.clone());
            return;
        }
        if index == candidates.len() || candidates[index] > remain {
            return;
        }

        path.push(candidates[index]);
        Self::dfs(index, remain - candidates[index], candidates, path, out);
        path.pop();
        Self::dfs(index + 1, remain, candidates, path, out);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut values: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for value in &mut values {
            value.sort_unstable();
        }
        values.sort();
        values
    }

    fn assert_all_solutions(candidates: Vec<i32>, target: i32, expected: Vec<Vec<i32>>) {
        let expected = normalize(expected);
        assert_eq!(normalize(NaiveSolution::combination_sum(candidates.clone(), target)), expected);
        assert_eq!(normalize(Solution::combination_sum(candidates.clone(), target)), expected);
        assert_eq!(normalize(HelperSolution::combination_sum(candidates, target)), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]);
    }

    #[test]
    fn multiple_branch_case() {
        assert_all_solutions(vec![2, 3, 5], 8, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    }
}
