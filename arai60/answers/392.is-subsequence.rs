/*
この問題で鍛えること:
- アルゴリズム: greedy と two pointers
- データ構造: 多クエリ向けの位置 index
- Rust での練習ポイント: byte 列走査、`partition_point`、補助関数への切り出し
- コード設計: 単発クエリの最短実装と、多クエリ想定の設計差を比較する

問題の本質:
- `s` の各文字に対して、`t` の中で左から順に対応位置を見つけられるかを判定したい
- ここで重要なのは、1 度使った位置より左へ戻る必要がないこと
- つまり、最左で一致する位置を greedily に選んでいけば、それ以外を試す必要がない

解法候補:
1. `t` の中を都度スキャンして、次の一致位置を探す
   - 時間計算量: O(|s| * |t|)
   - 空間計算量: O(1)
   - 採用/非採用理由: 素朴だが、同じ領域を何度もなめる
2. two pointers
   - 時間計算量: O(|t|)
   - 空間計算量: O(1)
   - 採用/非採用理由: 単発クエリなら最も素直で、本命
3. 文字ごとの出現位置を前処理して二分探索
   - 時間計算量: 前処理 O(|t|)、照会 O(|s| log |t|)
   - 空間計算量: O(|t|)
   - 採用/非採用理由: 多数クエリなら有力だが、単発では少し重い

採用解法の説明:
- `s` と `t` を左から見る 2 本のポインタを持つ
- `t` を 1 文字ずつ見て、今 `s` が必要としている文字と一致したら `s` 側を進める
- この greedy が正しい理由は、より右の一致位置を選ぶより、最左の一致位置を選んだ方が後続の余地が広いから
- 不変条件は「`s[..matched]` は `t[..current]` の subsequence としてすでに埋まっている」こと

Rust観点の解説:
- lower-case English 前提なので、`chars()` より `bytes()` の方が単純で速い
- `partition_point` を使うと、「前回使った位置より右で最初の候補」をきれいに取れる
- `String` を所有で受けても、中では `as_bytes()` で借用して読める

いいコード観点の解説:
- 本命実装は、`matched` という名前で「何文字進んだか」を明示すると読みやすい
- 整理版では、「次に必要な文字が取れたら進める」という責務を補助関数へ切り出す価値がある
- 多クエリ向けの別解は、問題条件が変わったときの設計の伸びしろを見せる

落とし穴:
- 一致したときに `t` 側ではなく `s` 側だけ進めればよいのに、両方の扱いを混乱させる
- 空文字列は常に subsequence であることを忘れる
- 多クエリ向け解法で、前回位置より「厳密に右」を探す条件を崩す
- `chars()` と `bytes()` を混ぜて index の意味を曖昧にする
*/

struct Solution;
struct ScanFromPositionSolution;
struct IndexedPositionsSolution;
struct RefinedSolution;

impl ScanFromPositionSolution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let source = s.as_bytes();
        let target = t.as_bytes();
        let mut start = 0usize;

        for &needed in source {
            let mut found = false;

            for index in start..target.len() {
                if target[index] == needed {
                    start = index + 1;
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        true
    }
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let source = s.as_bytes();
        let target = t.as_bytes();
        let mut matched = 0usize;

        // `matched` より左の `s` は、すでに `t` のどこかで順に対応づけられている。
        for &current in target {
            if matched == source.len() {
                return true;
            }

            if source[matched] == current {
                matched += 1;
            }
        }

        matched == source.len()
    }
}

impl IndexedPositionsSolution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let positions = Self::build_positions(t.as_bytes());
        let mut last_used = 0usize;

        for &needed in s.as_bytes() {
            let indices = &positions[(needed - b'a') as usize];
            let next_slot = indices.partition_point(|&index| index < last_used);
            if next_slot == indices.len() {
                return false;
            }
            last_used = indices[next_slot] + 1;
        }

        true
    }

    fn build_positions(target: &[u8]) -> Vec<Vec<usize>> {
        let mut positions = vec![Vec::new(); 26];

        for (index, &byte) in target.iter().enumerate() {
            positions[(byte - b'a') as usize].push(index);
        }

        positions
    }
}

impl RefinedSolution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let source = s.as_bytes();
        let target = t.as_bytes();
        let mut matched = 0usize;

        for &current in target {
            if Self::advance_if_matches(source, &mut matched, current) {
                return true;
            }
        }

        matched == source.len()
    }

    fn advance_if_matches(source: &[u8], matched: &mut usize, current: u8) -> bool {
        if *matched == source.len() {
            return true;
        }

        if source[*matched] == current {
            *matched += 1;
        }

        *matched == source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(s: &str, t: &str, expected: bool) {
        assert_eq!(
            ScanFromPositionSolution::is_subsequence(s.to_string(), t.to_string()),
            expected
        );
        assert_eq!(Solution::is_subsequence(s.to_string(), t.to_string()), expected);
        assert_eq!(
            IndexedPositionsSolution::is_subsequence(s.to_string(), t.to_string()),
            expected
        );
        assert_eq!(
            RefinedSolution::is_subsequence(s.to_string(), t.to_string()),
            expected
        );
    }

    #[test]
    fn basic_true_case() {
        assert_all_solutions("abc", "ahbgdc", true);
    }

    #[test]
    fn missing_character_returns_false() {
        assert_all_solutions("axc", "ahbgdc", false);
    }

    #[test]
    fn empty_string_is_always_a_subsequence() {
        assert_all_solutions("", "ahbgdc", true);
    }

    #[test]
    fn repeated_characters_respect_order() {
        assert_all_solutions("aaa", "baaab", true);
        assert_all_solutions("aaaa", "baaab", false);
    }
}
