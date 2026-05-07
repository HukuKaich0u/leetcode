/*
この問題で鍛えること:
- アルゴリズム: running minimum による状態要約
- データ構造: 追加構造なしの 1 pass
- Rust での練習ポイント: 所有権を動かす走査、`min` / `max` 更新
- コード設計: 全探索から「その日までの最良要約」に圧縮する感覚

問題の本質:
- `i` 日目で売ると決めたとき、必要なのは `i` 日目以前の最安値だけ
- 未来を予測する必要はなく、過去をどう要約するかが勝負
- 問題は「最良の買い日と売り日を全探索する」ではなく、「過去最安値と現在利幅を更新し続ける」問題

解法候補:
1. 全組を試す
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 発想の出発点としてはよいが、過去の要約を使えていない
2. running minimum
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 状態が最小で、本命
3. DP 風に `hold` / `cash` を持つ
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 一般化しやすいが、この問題単体では少し大げさ

採用解法の説明:
- `min_price` を「ここまでに見た最安値」とする
- 各日の価格 `price` に対して、今売ったときの利益は `price - min_price`
- その利益の最大値を `best` に保つ
- その後で `min_price` を更新してもよいが、実装ではどちらが先でも `best` が壊れないように整理する

Rust観点の解説:
- `for price in prices` で所有権を動かしてよい問題なので、`iter()` ではなく値で回しても素直
- `min_price` と `best` はどちらもスカラー値なので、状態が軽い
- 整理版では「価格を 1 つ読むたびの状態更新」を関数に切り出せる

いいコード観点の解説:
- `min_price` と `best` の 2 状態だけに絞ると、何を覚えていればよいかが明確
- DP 版は説明の一般化には役立つが、本問の主役にはしない
- 変数名を `buy` `sell` にすると日付そのものと混ざりやすいので、役割で名前を付ける

落とし穴:
- 最安値の更新より前に利益を計算するか後に計算するかで混乱する
- 1 回売買なのに、複数の上昇区間を足してしまう
- 解が無いときに負の利益を返してしまう
*/

use std::cmp::{max, min};

struct Solution;
struct BruteForceSolution;
struct DpStyleSolution;
struct RefinedSolution;

impl BruteForceSolution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut best = 0;

        for buy_day in 0..prices.len() {
            for sell_day in buy_day + 1..prices.len() {
                best = best.max(prices[sell_day] - prices[buy_day]);
            }
        }

        best
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut best = 0;

        for price in prices {
            min_price = min(min_price, price);
            best = max(best, price - min_price);
        }

        best
    }
}

impl DpStyleSolution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut hold = i32::MIN;
        let mut cash = 0;

        for price in prices {
            hold = max(hold, -price);
            cash = max(cash, hold + price);
        }

        cash
    }
}

impl RefinedSolution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::MAX;
        let mut best = 0;

        for price in prices {
            Self::update_state(price, &mut min_price, &mut best);
        }

        best
    }

    fn update_state(price: i32, min_price: &mut i32, best: &mut i32) {
        *min_price = min(*min_price, price);
        *best = max(*best, price - *min_price);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(prices: Vec<i32>, expected: i32) {
        assert_eq!(BruteForceSolution::max_profit(prices.clone()), expected);
        assert_eq!(Solution::max_profit(prices.clone()), expected);
        assert_eq!(DpStyleSolution::max_profit(prices.clone()), expected);
        assert_eq!(RefinedSolution::max_profit(prices), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![7, 1, 5, 3, 6, 4], 5);
    }

    #[test]
    fn no_profit_case() {
        assert_all_solutions(vec![7, 6, 4, 3, 1], 0);
    }

    #[test]
    fn late_minimum_case() {
        assert_all_solutions(vec![9, 8, 1, 10], 9);
    }
}
