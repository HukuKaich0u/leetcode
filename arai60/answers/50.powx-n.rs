/*
この問題で鍛えること:
- アルゴリズム: binary exponentiation
- データ構造: 追加構造なしの反復/再帰
- Rust での練習ポイント: `i32::MIN` 回避のための `i64`、負指数処理、helper 関数
- コード設計: 素朴な n 回掛け算から、指数を半分に折る設計へ移る感覚

問題の本質:
- `x^n` をそのまま `n` 回掛ける必要はない
- `n` が偶数なら `x^n = (x^(n/2))^2`、奇数ならそこに `x` を 1 回掛ければよい
- つまり問題は累乗ではなく、同じ部分結果を再利用できる分割統治の問題

解法候補:
1. 素朴な反復
   - 時間計算量: O(|n|)
   - 空間計算量: O(1)
   - 採用/非採用理由: 定義は直感的だが大きい指数で遅い
2. 反復 binary exponentiation
   - 時間計算量: O(log |n|)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命。実務でも壊れにくい
3. 再帰 binary exponentiation
   - 時間計算量: O(log |n|)
   - 空間計算量: O(log |n|)
   - 採用/非採用理由: 数式に近い比較対象として有益

採用解法の説明:
- 指数を 2 進数で見て、下位 bit から処理する
- 現在の bit が 1 なら答えへ `base` を掛ける
- 毎回 `base *= base` して、指数を半分に落とす
- 負指数は `x^-n = 1 / x^n` に直すが、`i32::MIN` の絶対値は overflow するので `i64` へ上げる

Rust観点の解説:
- `n as i64` にしてから絶対値を扱うと `i32::MIN` を安全に処理できる
- 反復版は stack を使わずに済むので、本命として安定
- 再帰版は `pow_positive` を helper に切ると意味が明確

いいコード観点の解説:
- 本命実装では「指数を半分に折る」ロジックが一直線に見える
- 再帰版は式の美しさがあるが、反復版の方が境界条件の制御がしやすい
- 素朴版を併記すると、なぜ O(log n) が必要かを比較で示せる

落とし穴:
- `i32::MIN.abs()` を直接取って overflow する
- 負指数で `x` を逆数にしてから指数を反転する順序を崩す
- 奇数 bit の掛け算を忘れる
*/

struct Solution;
struct NaiveSolution;
struct RecursiveSolution;
struct HelperSolution;

impl NaiveSolution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let exp = n as i64;
        let times = exp.unsigned_abs();
        let mut answer = 1.0;

        for _ in 0..times {
            answer *= x;
        }

        if exp >= 0 {
            answer
        } else {
            1.0 / answer
        }
    }
}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn fast_pow(mut base: f64, mut exp: i64) -> f64 {
            let mut answer = 1.0;
            while exp > 0 {
                if exp % 2 == 1 {
                    answer *= base;
                }
                base *= base;
                exp /= 2;
            }
            answer
        }

        let exp = n as i64;
        if exp >= 0 {
            fast_pow(x, exp)
        } else {
            1.0 / fast_pow(x, -exp)
        }
    }
}

impl RecursiveSolution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let exp = n as i64;
        if exp >= 0 {
            Self::pow_positive(x, exp)
        } else {
            1.0 / Self::pow_positive(x, -exp)
        }
    }

    fn pow_positive(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let half = Self::pow_positive(x, n / 2);
        if n % 2 == 0 {
            half * half
        } else {
            half * half * x
        }
    }
}

impl HelperSolution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let exponent = n as i64;
        let magnitude = exponent.abs();
        let answer = Self::pow_positive(x, magnitude);
        if exponent >= 0 { answer } else { 1.0 / answer }
    }

    fn pow_positive(mut base: f64, mut exp: i64) -> f64 {
        let mut answer = 1.0;
        while exp > 0 {
            if exp & 1 == 1 {
                answer *= base;
            }
            base *= base;
            exp >>= 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-10,
            "actual={}, expected={}",
            actual,
            expected
        );
    }

    fn assert_all_solutions(x: f64, n: i32, expected: f64) {
        assert_close(NaiveSolution::my_pow(x, n), expected);
        assert_close(Solution::my_pow(x, n), expected);
        assert_close(RecursiveSolution::my_pow(x, n), expected);
        assert_close(HelperSolution::my_pow(x, n), expected);
    }

    #[test]
    fn positive_exponent() {
        assert_all_solutions(2.0, 10, 1024.0);
    }

    #[test]
    fn negative_exponent() {
        assert_all_solutions(2.0, -2, 0.25);
    }

    #[test]
    fn zero_exponent() {
        assert_all_solutions(3.0, 0, 1.0);
    }

    #[test]
    fn min_exponent_edge_case() {
        // このケースは `i32::MIN` の絶対値が大きすぎるので、
        // 素朴実装ではなく overflow 回避込みの本命/整理版を検証する。
        assert_close(Solution::my_pow(1.0, i32::MIN), 1.0);
        assert_close(RecursiveSolution::my_pow(1.0, i32::MIN), 1.0);
        assert_close(HelperSolution::my_pow(1.0, i32::MIN), 1.0);
    }
}
