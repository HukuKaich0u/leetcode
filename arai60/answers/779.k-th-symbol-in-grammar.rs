/*
この問題で鍛えること:
- アルゴリズム: recursion, divide-and-conquer, bit parity
- データ構造: 追加構造なし
- Rust での練習ポイント: bit 演算、helper 再帰
- コード設計: 生成規則を親子関係へ写す力

問題の本質:
- 行全体を生成する必要はない
- `k` 番目の記号が前半にいるか後半にいるかで、1 つ上の親から値が決まる
- さらに突き詰めると、`k-1` の 1 ビット数の偶奇でも決まる

解法候補:
1. 行を順に生成する
   - 採用/非採用理由: 比較用にはなるが無駄が大きい
2. 親をたどる再帰
   - 採用/非採用理由: 本命
3. popcount parity
   - 採用/非採用理由: 数式化した整理版

採用解法の説明:
- 長さ `2^(n-1)` の列の前半は親行そのまま、後半は親行の反転
- `k` が前半なら親の値と同じ、後半なら親の値を反転する
- したがって `(n, k)` を `(n-1, parent_k)` へ縮めていける

Rust観点の解説:
- `1 << (n - 2)` で半分の長さを出せる
- parity 版は `count_ones()` を使うと一気に書ける
- 再帰版と parity 版を並べると、同じ問題を式まで圧縮する感覚が学べる

いいコード観点の解説:
- 本命実装は問題文の生成規則との対応が最も強い
- 数学的別解を併記すると、実装の背後にある構造理解が深まる
- 「全部作らない」が設計判断として明確に見える問題

落とし穴:
- `k` を 0-index と 1-index で混同する
- 後半に入ったときの反転を忘れる
- 行全体を本当に生成して指数爆発する
*/

struct Solution;
struct NaiveSolution;
struct ParitySolution;

impl NaiveSolution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut row = vec![0];
        for _ in 2..=n {
            let mut next = Vec::with_capacity(row.len() * 2);
            for value in row {
                next.push(value);
                next.push(1 - value);
            }
            row = next;
        }
        row[k as usize - 1]
    }
}

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        let half = 1 << (n - 2);
        if k <= half {
            Self::kth_grammar(n - 1, k)
        } else {
            1 - Self::kth_grammar(n - 1, k - half)
        }
    }
}

impl ParitySolution {
    pub fn kth_grammar(_n: i32, k: i32) -> i32 {
        ((k - 1).count_ones() % 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(n: i32, k: i32, expected: i32) {
        assert_eq!(NaiveSolution::kth_grammar(n, k), expected);
        assert_eq!(Solution::kth_grammar(n, k), expected);
        assert_eq!(ParitySolution::kth_grammar(n, k), expected);
    }

    #[test]
    fn first_rows() {
        assert_all_solutions(1, 1, 0);
        assert_all_solutions(2, 1, 0);
        assert_all_solutions(2, 2, 1);
    }

    #[test]
    fn deeper_case() {
        assert_all_solutions(4, 5, 1);
    }
}
