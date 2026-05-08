/*
この問題で鍛えること:
- アルゴリズム: sliding window
- データ構造: HashSet と HashMap の使い分け
- Rust での練習ポイント: 文字列走査、`HashMap<char, usize>`、window 境界の更新
- コード設計: shrink しながら保つ版と、最後の出現位置で一気に飛ばす版の比較

問題の本質:
- 求めたいのは「重複を含まない最大 window」
- 文字列の部分列を毎回作る必要はなく、window が妥当かどうかを局所情報で判定できればよい
- つまり、文字列問題というより「重複なし区間を維持する sliding window」の問題

解法候補:
1. 全ての開始位置から重複が出るまで伸ばす
   - 時間計算量: O(n^2)
   - 空間計算量: O(σ)
   - 採用/非採用理由: 素朴だが、window の再利用がない
2. HashSet で重複が無くなるまで左を縮める
   - 時間計算量: O(n)
   - 空間計算量: O(σ)
   - 採用/非採用理由: window の基本として良い比較対象
3. 最後の出現位置を持ち、左端を一気に飛ばす
   - 時間計算量: O(n)
   - 空間計算量: O(σ)
   - 採用/非採用理由: 同じ window でも更新が鋭く、本命

採用解法の説明:
- `last[ch] = その文字を最後に見た位置` を持つ
- 右端 `right` で文字 `ch` を見たとき、もし `ch` が現在 window 内にあるなら、左端は `last[ch] + 1` 以上に進める必要がある
- ただし左端は後ろに戻ってはいけないので、`left = max(left, last[ch] + 1)` とする
- この更新で、常に `s[left..=right]` は重複なしを保つ

Rust観点の解説:
- `chars().enumerate()` を使うと、Unicode 文字単位で index を付けて扱える
- `if let Some(&prev)` で map から参照を外し、`left.max(prev + 1)` にそのまま使う
- HashSet 版では `bytes()` の方が単純だが、本命と合わせて `chars()` に寄せてもよい

いいコード観点の解説:
- `left` が「現在の有効 window の左端」だと名前で分かるようにする
- 本命実装は 1 回の走査で、何を更新しているかが一直線に読める
- HashSet 版を併記すると、「window を shrink する標準形」と「最後の位置を覚えて飛ばす改善形」の差が見える

落とし穴:
- `left = prev + 1` としてしまい、すでに前進した `left` を後退させる
- 文字列を byte と char で混ぜて index の意味を崩す
- HashSet 版で左端の文字を消し忘れて無限ループする
*/

use std::collections::{HashMap, HashSet};

struct Solution;
struct BruteForceSolution;
struct SetWindowSolution;
struct RefinedSolution;

impl BruteForceSolution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut best = 0usize;

        for left in 0..chars.len() {
            let mut seen = HashSet::new();
            for &ch in chars.iter().skip(left) {
                if !seen.insert(ch) {
                    break;
                }
                best = best.max(seen.len());
            }
        }

        best as i32
    }
}

impl SetWindowSolution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut seen = HashSet::new();
        let mut left = 0usize;
        let mut best = 0usize;

        for right in 0..chars.len() {
            while seen.contains(&chars[right]) {
                seen.remove(&chars[left]);
                left += 1;
            }
            seen.insert(chars[right]);
            best = best.max(right - left + 1);
        }

        best as i32
    }
}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = HashMap::new();
        let mut left = 0usize;
        let mut best = 0usize;

        for (right, ch) in s.chars().enumerate() {
            if let Some(&previous_index) = last.get(&ch) {
                left = left.max(previous_index + 1);
            }
            best = best.max(right - left + 1);
            last.insert(ch, right);
        }

        best as i32
    }
}

impl RefinedSolution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = HashMap::new();
        let mut left = 0usize;
        let mut best = 0usize;

        for (right, ch) in s.chars().enumerate() {
            Self::advance_left(&last, ch, &mut left);
            best = best.max(right - left + 1);
            last.insert(ch, right);
        }

        best as i32
    }

    fn advance_left(last: &HashMap<char, usize>, ch: char, left: &mut usize) {
        if let Some(&previous_index) = last.get(&ch) {
            *left = (*left).max(previous_index + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(s: &str, expected: i32) {
        assert_eq!(
            BruteForceSolution::length_of_longest_substring(s.to_string()),
            expected
        );
        assert_eq!(
            SetWindowSolution::length_of_longest_substring(s.to_string()),
            expected
        );
        assert_eq!(Solution::length_of_longest_substring(s.to_string()), expected);
        assert_eq!(
            RefinedSolution::length_of_longest_substring(s.to_string()),
            expected
        );
    }

    #[test]
    fn basic_case() {
        assert_all_solutions("abcabcbb", 3);
    }

    #[test]
    fn repeated_characters_case() {
        assert_all_solutions("pwwkew", 3);
    }

    #[test]
    fn empty_string() {
        assert_all_solutions("", 0);
    }

    #[test]
    fn all_same_character() {
        assert_all_solutions("bbbbb", 1);
    }
}
