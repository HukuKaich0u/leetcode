/*
この問題で鍛えること:
- アルゴリズム: greedy の等価変形
- データ構造: 状態変数だけで書く DP との比較
- Rust での練習ポイント: `windows(2)`、反復和、状態更新
- コード設計: 1 回売買の問題との違いをコードで言語化する

問題の本質:
- 複数回売買できるので、上昇区間を 1 回で取り切っても、日ごとの差分を全部足しても利益は同じ
- つまり「いつ谷で買って山で売るか」を細かく追う必要はなく、正の差分だけ集めればよい
- この等価変形を見抜けるかが核心

解法候補:
1. 谷と山を明示的に探す
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 発想の導入としてよいが、本命より少し長い
2. 正の差分を全部足す
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 最も簡潔で、本命
3. DP の `hold` / `cash`
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 手数料や cooldown 付きへ一般化しやすい

採用解法の説明:
- 連続する上昇区間 `a < b < c` があるとき、`(c - a) = (b - a) + (c - b)`
- したがって、値上がりした日ごとの差分を全部足しても、谷で買って山で売る利益と一致する
- よって `prices[i + 1] - prices[i]` が正なら加算するだけでよい

Rust観点の解説:
- `windows(2)` を使うと隣接 2 要素差分を素直に書ける
- DP 版では `hold` と `cash` の更新順が重要なので、前状態を退避する
- valley-peak 版は while ループ中心で、境界条件の扱いを練習しやすい

いいコード観点の解説:
- 本命実装は短いが、なぜそれでよいかの解説がないと価値が落ちる
- valley-peak 版は概念が見えやすく、初学者には良い比較対象
- DP 版は一般化の入口として有益だが、本問だけなら本命より重い

落とし穴:
- 1 回売買の解法をそのまま流用してしまう
- valley と peak を探す実装で index 更新が絡み合う
- DP 版で `cash` 更新後の値を使って `hold` を更新してしまう
*/

use std::cmp::max;

struct Solution;
struct ValleyPeakSolution;
struct DpStateSolution;
struct RefinedSolution;

impl ValleyPeakSolution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut day = 0usize;
        let mut profit = 0;

        while day + 1 < prices.len() {
            while day + 1 < prices.len() && prices[day] >= prices[day + 1] {
                day += 1;
            }
            let valley = prices[day];

            while day + 1 < prices.len() && prices[day] <= prices[day + 1] {
                day += 1;
            }
            let peak = prices[day];

            profit += peak - valley;
        }

        profit
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).map(|pair| max(0, pair[1] - pair[0])).sum()
    }
}

impl DpStateSolution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut hold = i32::MIN;
        let mut cash = 0;

        for price in prices {
            let previous_cash = cash;
            cash = max(cash, hold + price);
            hold = max(hold, previous_cash - price);
        }

        cash
    }
}

impl RefinedSolution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        for pair in prices.windows(2) {
            profit += Self::positive_gain(pair[0], pair[1]);
        }

        profit
    }

    fn positive_gain(previous: i32, current: i32) -> i32 {
        max(0, current - previous)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(prices: Vec<i32>, expected: i32) {
        assert_eq!(ValleyPeakSolution::max_profit(prices.clone()), expected);
        assert_eq!(Solution::max_profit(prices.clone()), expected);
        assert_eq!(DpStateSolution::max_profit(prices.clone()), expected);
        assert_eq!(RefinedSolution::max_profit(prices), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![7, 1, 5, 3, 6, 4], 7);
    }

    #[test]
    fn monotonic_increase() {
        assert_all_solutions(vec![1, 2, 3, 4, 5], 4);
    }

    #[test]
    fn no_profit_case() {
        assert_all_solutions(vec![7, 6, 4, 3, 1], 0);
    }
}
