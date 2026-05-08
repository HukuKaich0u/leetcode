/*
この問題で鍛えること:
- アルゴリズム: parser の逐次実装
- データ構造: 追加構造なし
- Rust での練習ポイント: byte 列走査、overflow 判定、helper による責務分離
- コード設計: 仕様を処理順へ落とし込む力

問題の本質:
- 文字列全体を数値へ変換する問題ではなく、「仕様で許された prefix だけ読む」問題
- 順序は固定で、空白スキップ -> 符号 -> 数字列 -> overflow clamp
- parser は仕様をそのまま機械化するのが最も強い

解法候補:
1. 文字を集めて後から parse する
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 文字列再構築が余計
2. 走査しながら値を更新する
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命
3. digit 読み取りを helper 化する
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: parser の段階が見えやすい整理版

採用解法の説明:
- 先頭空白を飛ばす
- 次に任意の符号を 1 文字だけ読む
- 以降は数字が続く限り `value = value * 10 + digit` を繰り返す
- 各桁で overflow を監視し、範囲外なら即 clamp して返す

Rust観点の解説:
- `as_bytes()` で byte 単位に見ると ASCII 仕様をまっすぐ書ける
- `i64` へ広げてから `i32` へ clamp すると overflow を避けやすい
- helper 版では index を `&mut usize` で受けて parser の進行を共有する

いいコード観点の解説:
- parser は「仕様の順番」と「コードの順番」が揃っているほど読みやすい
- overflow 判定を後回しにすると壊れやすいので、桁追加ごとに判定する方が堅い
- 状態機械にしすぎなくても、処理段階が分かれていれば十分強いコードになる

落とし穴:
- 符号を 2 文字以上読んでしまう
- overflow を最後にだけ判定して途中で壊れる
- 数字が 1 文字もないケースを 0 以外で返してしまう
*/

struct Solution;
struct NaiveSolution;
struct HelperSolution;

impl NaiveSolution {
    pub fn my_atoi(s: String) -> i32 {
        let trimmed = s.trim_start();
        let mut chars = trimmed.chars().peekable();
        let mut token = String::new();

        if matches!(chars.peek(), Some('+') | Some('-')) {
            token.push(chars.next().unwrap());
        }
        while let Some(&ch) = chars.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            token.push(chars.next().unwrap());
        }

        if token.is_empty() || token == "+" || token == "-" {
            return 0;
        }

        token
            .parse::<i64>()
            .map(|value| value.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
            .unwrap_or_else(|_| {
                if token.starts_with('-') {
                    i32::MIN
                } else {
                    i32::MAX
                }
            })
    }
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut index = 0;

        while index < bytes.len() && bytes[index] == b' ' {
            index += 1;
        }

        let mut sign = 1i64;
        if index < bytes.len() && (bytes[index] == b'+' || bytes[index] == b'-') {
            if bytes[index] == b'-' {
                sign = -1;
            }
            index += 1;
        }

        let mut value = 0i64;
        // 1 桁読むたびに clamp 判定を入れると、中間 overflow を避けられる。
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            value = value * 10 + (bytes[index] - b'0') as i64;
            let signed = value * sign;
            if signed > i32::MAX as i64 {
                return i32::MAX;
            }
            if signed < i32::MIN as i64 {
                return i32::MIN;
            }
            index += 1;
        }

        (value * sign) as i32
    }
}

impl HelperSolution {
    pub fn my_atoi(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut index = 0;
        Self::skip_spaces(bytes, &mut index);
        let sign = Self::read_sign(bytes, &mut index);
        Self::read_digits(bytes, &mut index, sign)
    }

    fn skip_spaces(bytes: &[u8], index: &mut usize) {
        while *index < bytes.len() && bytes[*index] == b' ' {
            *index += 1;
        }
    }

    fn read_sign(bytes: &[u8], index: &mut usize) -> i64 {
        if *index < bytes.len() && (bytes[*index] == b'+' || bytes[*index] == b'-') {
            let sign = if bytes[*index] == b'-' { -1 } else { 1 };
            *index += 1;
            sign
        } else {
            1
        }
    }

    fn read_digits(bytes: &[u8], index: &mut usize, sign: i64) -> i32 {
        let mut value = 0i64;
        while *index < bytes.len() && bytes[*index].is_ascii_digit() {
            value = value * 10 + (bytes[*index] - b'0') as i64;
            let signed = value * sign;
            if signed > i32::MAX as i64 {
                return i32::MAX;
            }
            if signed < i32::MIN as i64 {
                return i32::MIN;
            }
            *index += 1;
        }
        (value * sign) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(input: &str, expected: i32) {
        assert_eq!(NaiveSolution::my_atoi(input.to_string()), expected);
        assert_eq!(Solution::my_atoi(input.to_string()), expected);
        assert_eq!(HelperSolution::my_atoi(input.to_string()), expected);
    }

    #[test]
    fn parses_basic_signed_value() {
        assert_all_solutions("   -42", -42);
    }

    #[test]
    fn stops_at_first_non_digit() {
        assert_all_solutions("4193 with words", 4193);
    }

    #[test]
    fn clamps_positive_overflow() {
        assert_all_solutions("91283472332", i32::MAX);
    }

    #[test]
    fn clamps_negative_overflow() {
        assert_all_solutions("-91283472332", i32::MIN);
    }
}
