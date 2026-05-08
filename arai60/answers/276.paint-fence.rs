/*
この問題で鍛えること:
- アルゴリズム: counting DP
- データ構造: 定数個の状態
- Rust での練習ポイント: 状態意味を変数名へ落とす
- コード設計: 自然言語の条件を DP 状態へ分解する力

問題の本質:
- 重要なのは「直前 2 本が同じ色で終わっているか、違う色で終わっているか」
- 新しい柵を塗るとき、許される色数はこの状態によって変わる
- 全履歴ではなく、その 2 状態だけ持てば十分

解法候補:
1. 全列挙
   - 採用/非採用理由: 指数時間
2. DP 配列で same/diff を持つ
   - 採用/非採用理由: 意味が見えやすい
3. same/diff を定数個に圧縮
   - 採用/非採用理由: 本命

採用解法の説明:
- `same`: 最後の 2 本が同色で終わる塗り方数
- `diff`: 最後の 2 本が異色で終わる塗り方数
- 次の柵を塗るとき
  - `next_same = diff`
  - `next_diff = (same + diff) * (k - 1)`

Rust観点の解説:
- 状態名を `same/diff` と明示すると式の意味が崩れにくい
- `match n` で基底ケースを先に処理してループを短く保てる
- DP 配列版も残すと、圧縮前との対応が追いやすい

いいコード観点の解説:
- 数え上げ DP は「状態が何を数えているか」を口で言えるかが最重要
- 本命実装は式だけ見ると短いので、コメントや周辺解説で意味を補うのが大事
- 自然言語の条件分解がうまくできると、実装はむしろ短くなる

落とし穴:
- `same` と `diff` の意味を取り違える
- `n == 1` や `n == 2` の基底ケースを落とす
- `(same + diff) * (k - 1)` を `diff * (k - 1)` と誤る
*/

struct Solution;
struct DpArraySolution;
struct RecursiveMemoSolution;

impl RecursiveMemoSolution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        fn dfs(i: i32, same_prev: bool, k: i32, memo: &mut Vec<[Option<i32>; 2]>) -> i32 {
            if i == 0 {
                return 1;
            }
            let flag = same_prev as usize;
            if let Some(value) = memo[i as usize][flag] {
                return value;
            }
            let value = if same_prev {
                dfs(i - 1, false, k, memo) * (k - 1)
            } else {
                dfs(i - 1, true, k, memo) + dfs(i - 1, false, k, memo) * (k - 1)
            };
            memo[i as usize][flag] = Some(value);
            value
        }

        match n {
            0 => 0,
            1 => k,
            2 => k * k,
            _ => {
                let mut memo = vec![[None, None]; (n - 1) as usize + 1];
                k * dfs(n - 1, false, k, &mut memo)
            }
        }
    }
}

impl DpArraySolution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        match n {
            0 => 0,
            1 => k,
            _ => {
                let mut same = vec![0; n as usize + 1];
                let mut diff = vec![0; n as usize + 1];
                same[2] = k;
                diff[2] = k * (k - 1);
                for i in 3..=n as usize {
                    same[i] = diff[i - 1];
                    diff[i] = (same[i - 1] + diff[i - 1]) * (k - 1);
                }
                same[n as usize] + diff[n as usize]
            }
        }
    }
}

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        match n {
            0 => return 0,
            1 => return k,
            _ => {}
        }

        let mut same = k;
        let mut diff = k * (k - 1);
        for _ in 3..=n {
            let next_same = diff;
            let next_diff = (same + diff) * (k - 1);
            same = next_same;
            diff = next_diff;
        }
        same + diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(n: i32, k: i32, expected: i32) {
        assert_eq!(RecursiveMemoSolution::num_ways(n, k), expected);
        assert_eq!(DpArraySolution::num_ways(n, k), expected);
        assert_eq!(Solution::num_ways(n, k), expected);
    }

    #[test]
    fn zero_posts_has_zero_ways() {
        assert_all_solutions(0, 3, 0);
    }

    #[test]
    fn one_post_has_k_choices() {
        assert_all_solutions(1, 3, 3);
    }

    #[test]
    fn three_posts_two_colors() {
        assert_all_solutions(3, 2, 6);
    }
}
