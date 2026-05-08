/*
この問題で鍛えること:
- アルゴリズム: stack
- データ構造: LIFO 構造でネストを追う考え方
- Rust での練習ポイント: `Vec` を stack として使う、`match`、補助関数化
- コード設計: 置換的な愚直案から stack の本質へ寄せる感覚

問題の本質:
- 括弧の整合性は「最後に開いたものが最初に閉じる」ことで決まる
- つまり、未解決の開き括弧を LIFO で管理できれば十分
- 種類ごとの個数だけでは順序違いを検出できない

解法候補:
1. `()` `[]` `{}` を文字列から消し続ける
   - 時間計算量: O(n^2)
   - 空間計算量: O(n)
   - 採用/非採用理由: 仕様の理解にはよいが非効率
2. stack
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: ネスト構造を最も自然に表せるので本命
3. stack を補助関数で整理した版
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 対応判定の責務を切り出して読みやすくする

採用解法の説明:
- 開き括弧は stack へ積む
- 閉じ括弧を読んだら、stack の先頭と種類が対応しているかを確認する
- 対応しないか、stack が空なら即 false
- 走査後に stack が空なら、すべて正しく閉じられている

Rust観点の解説:
- `Vec<char>` は `push` / `pop` があるので stack としてそのまま使える
- `stack.pop() != Some('(')` の形で、空 stack と種類不一致を同時に扱える
- 整理版では「閉じ括弧に対応する開き括弧」を関数に切り出すと `match` が薄くなる

いいコード観点の解説:
- 本命実装は処理の流れが一直線で、読みやすい
- 条件分岐が増えやすい問題なので、整理版で「対応表現」を切り出す価値がある
- 置換版は比較対象としては有効だが、本番コードにはしない

落とし穴:
- `([)]` のような順序違いを個数だけで誤判定する
- 走査後に stack が空か確認し忘れる
- 閉じ括弧から始まるケースで `pop()` の `None` を扱い損ねる
*/

struct Solution;
struct ReplacementSolution;
struct RefinedStackSolution;

impl ReplacementSolution {
    pub fn is_valid(s: String) -> bool {
        let mut current = s;

        loop {
            let next = current
                .replace("()", "")
                .replace("[]", "")
                .replace("{}", "");

            if next.len() == current.len() {
                return next.is_empty();
            }

            current = next;
        }
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.is_empty()
    }
}

impl RefinedStackSolution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for ch in s.chars() {
            if let Some(expected_open) = Self::matching_open(ch) {
                if stack.pop() != Some(expected_open) {
                    return false;
                }
            } else {
                stack.push(ch);
            }
        }

        stack.is_empty()
    }

    fn matching_open(ch: char) -> Option<char> {
        match ch {
            ')' => Some('('),
            ']' => Some('['),
            '}' => Some('{'),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(s: &str, expected: bool) {
        assert_eq!(ReplacementSolution::is_valid(s.to_string()), expected);
        assert_eq!(Solution::is_valid(s.to_string()), expected);
        assert_eq!(RefinedStackSolution::is_valid(s.to_string()), expected);
    }

    #[test]
    fn balanced_case() {
        assert_all_solutions("()[]{}", true);
    }

    #[test]
    fn wrong_order_case() {
        assert_all_solutions("([)]", false);
    }

    #[test]
    fn empty_string() {
        assert_all_solutions("", true);
    }

    #[test]
    fn unclosed_case() {
        assert_all_solutions("(()", false);
    }
}
