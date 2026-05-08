/*
この問題で鍛えること:
- アルゴリズム: prefix DP / memo DFS
- データ構造: HashSet
- Rust での練習ポイント: byte slice, `HashSet<Vec<u8>>`, memo
- コード設計: 状態を「位置」ではなく「prefix が切れるか」で持つ感覚

問題の本質:
- `dp[i]` を「先頭 i 文字が辞書語で分解可能か」と置く
- どこか `j < i` で切れて、`dp[j]` が true かつ `s[j..i]` が辞書語なら `dp[i] = true`
- 文字列 DP では、1 文字ではなく prefix 単位の状態が自然

解法候補:
1. memo DFS
   - 採用/非採用理由: 分割位置探索の形が見えやすい
2. bottom-up DP
   - 採用/非採用理由: 本命
3. 最大単語長で枝刈りする DP
   - 採用/非採用理由: 実務的な改善例として有益

採用解法の説明:
- `dp[0] = true` は空文字がすでに分解済みという意味
- 各 `i` についてすべての `j` を試し、切り方が 1 つでも成立すれば `dp[i] = true`
- これで「前半の正しさ」と「最後の 1 単語」を組み合わせられる

Rust観点の解説:
- `String` のまま index すると UTF-8 境界問題があるので、byte 列で見る方が安全
- `HashSet<Vec<u8>>` にしておくと slice 比較をそのまま書ける
- memo 版では `Option<bool>` で未計算状態を表す

いいコード観点の解説:
- 本命実装は状態意味がはっきりしていて説明しやすい
- helper 版で最大単語長を使うと、不要な `j` を減らせるという改善視点も学べる
- DP 問題は「何が既知なら次が決まるか」を言葉で定義できることが重要

落とし穴:
- `dp[i]` を「i 文字目まで」ではなく「i 文字目そのもの」と誤解する
- UTF-8 を文字単位で index しようとして壊す
- 空文字基底 `dp[0] = true` を忘れる
*/

use std::collections::HashSet;

struct Solution;
struct MemoSolution;
struct HelperSolution;

impl MemoSolution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        fn dfs(start: usize, bytes: &[u8], words: &HashSet<Vec<u8>>, memo: &mut [Option<bool>]) -> bool {
            if start == bytes.len() {
                return true;
            }
            if let Some(value) = memo[start] {
                return value;
            }

            for end in start + 1..=bytes.len() {
                if words.contains(&bytes[start..end]) && dfs(end, bytes, words, memo) {
                    memo[start] = Some(true);
                    return true;
                }
            }
            memo[start] = Some(false);
            false
        }

        let words: HashSet<Vec<u8>> = word_dict.into_iter().map(String::into_bytes).collect();
        let bytes = s.into_bytes();
        let mut memo = vec![None; bytes.len()];
        dfs(0, &bytes, &words, &mut memo)
    }
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let words: HashSet<Vec<u8>> = word_dict.into_iter().map(String::into_bytes).collect();
        let bytes = s.into_bytes();
        let mut dp = vec![false; bytes.len() + 1];
        dp[0] = true;

        for i in 1..=bytes.len() {
            for j in 0..i {
                if dp[j] && words.contains(&bytes[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[bytes.len()]
    }
}

impl HelperSolution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let max_len = word_dict.iter().map(|word| word.len()).max().unwrap_or(0);
        let words: HashSet<Vec<u8>> = word_dict.into_iter().map(String::into_bytes).collect();
        let bytes = s.into_bytes();
        let mut dp = vec![false; bytes.len() + 1];
        dp[0] = true;

        for i in 1..=bytes.len() {
            let start = i.saturating_sub(max_len);
            for j in start..i {
                if dp[j] && words.contains(&bytes[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[bytes.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(s: &str, words: Vec<&str>, expected: bool) {
        let dict: Vec<String> = words.iter().map(|word| word.to_string()).collect();
        assert_eq!(MemoSolution::word_break(s.to_string(), dict.clone()), expected);
        assert_eq!(Solution::word_break(s.to_string(), dict.clone()), expected);
        assert_eq!(HelperSolution::word_break(s.to_string(), dict), expected);
    }

    #[test]
    fn basic_true_case() {
        assert_all_solutions("leetcode", vec!["leet", "code"], true);
    }

    #[test]
    fn basic_false_case() {
        assert_all_solutions("catsandog", vec!["cats", "dog", "sand", "and", "cat"], false);
    }

    #[test]
    fn unicode_words_do_not_panic() {
        assert_all_solutions("あい", vec!["あ", "い"], true);
    }
}
