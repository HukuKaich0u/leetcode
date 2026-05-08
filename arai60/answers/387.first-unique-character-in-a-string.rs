/*
この問題で鍛えること:
- アルゴリズム: 2-pass counting
- データ構造: 固定長カウント配列と queue
- Rust での練習ポイント: `bytes()`、固定長配列、`VecDeque`
- コード設計: 統計取得と元順序復元を分ける考え方

問題の本質:
- 一意かどうかは出現回数を見ないと判断できない
- ただし「最初に現れる位置」は元の順序が必要
- だから 1 回目で頻度を数え、2 回目で順序に戻る分離が自然

解法候補:
1. 各文字について全体を数え直す
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 素朴だが再計算が多い
2. 2-pass counting
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 一番素直で、本命
3. queue で unique 候補を逐次保つ
   - 時間計算量: O(n)
   - 空間計算量: O(σ)
   - 採用/非採用理由: streaming 的な見方として価値がある

採用解法の説明:
- 1 回目で各文字の出現回数を数える
- 2 回目で左から見直し、回数 1 の最初の文字の index を返す
- 回数と順序を分離することで、ロジックが簡潔になる

Rust観点の解説:
- 問題が小文字英字なので `count[26]` を使うと `HashMap` より軽い
- `s.bytes().enumerate()` で index と byte を同時に取れる
- queue 版では `VecDeque<(usize, u8)>` を使って、先頭候補を保つ

いいコード観点の解説:
- 本命実装は 2 pass の役割が明確に分かれている
- queue 版を置くと、「逐次 unique 候補を保つ」という別の設計視点が見える
- 文字列問題でも、制約が固定なら配列で十分という判断を言語化できると強い

落とし穴:
- 1 回目で数えながら最初の unique を決めようとして壊す
- 存在しないときに `-1` を返し忘れる
- `char` と `u8` を混ぜて添字変換を崩す
*/

use std::collections::VecDeque;

struct Solution;
struct BruteForceSolution;
struct QueueStyleSolution;
struct RefinedSolution;

impl BruteForceSolution {
    pub fn first_uniq_char(s: String) -> i32 {
        let bytes = s.as_bytes();

        for (index, &current) in bytes.iter().enumerate() {
            let mut count = 0;
            for &other in bytes {
                if other == current {
                    count += 1;
                }
            }
            if count == 1 {
                return index as i32;
            }
        }

        -1
    }
}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = [0; 26];
        for byte in s.bytes() {
            count[(byte - b'a') as usize] += 1;
        }

        for (index, byte) in s.bytes().enumerate() {
            if count[(byte - b'a') as usize] == 1 {
                return index as i32;
            }
        }

        -1
    }
}

impl QueueStyleSolution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = [0; 26];
        let mut candidates = VecDeque::new();

        for (index, byte) in s.bytes().enumerate() {
            let slot = (byte - b'a') as usize;
            count[slot] += 1;
            candidates.push_back((index, byte));

            while let Some(&(_, front)) = candidates.front() {
                if count[(front - b'a') as usize] == 1 {
                    break;
                }
                candidates.pop_front();
            }
        }

        candidates.front().map(|&(index, _)| index as i32).unwrap_or(-1)
    }
}

impl RefinedSolution {
    pub fn first_uniq_char(s: String) -> i32 {
        let count = Self::count_bytes(s.as_bytes());
        for (index, byte) in s.bytes().enumerate() {
            if count[(byte - b'a') as usize] == 1 {
                return index as i32;
            }
        }
        -1
    }

    fn count_bytes(bytes: &[u8]) -> [i32; 26] {
        let mut count = [0; 26];
        for &byte in bytes {
            count[(byte - b'a') as usize] += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(s: &str, expected: i32) {
        assert_eq!(BruteForceSolution::first_uniq_char(s.to_string()), expected);
        assert_eq!(Solution::first_uniq_char(s.to_string()), expected);
        assert_eq!(QueueStyleSolution::first_uniq_char(s.to_string()), expected);
        assert_eq!(RefinedSolution::first_uniq_char(s.to_string()), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions("leetcode", 0);
    }

    #[test]
    fn late_unique_character() {
        assert_all_solutions("loveleetcode", 2);
    }

    #[test]
    fn missing_case() {
        assert_all_solutions("aabb", -1);
    }
}
