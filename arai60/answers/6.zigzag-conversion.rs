/*
この問題で鍛えること:
- アルゴリズム: simulation と周期ベースの見方
- データ構造: 行バッファ
- Rust での練習ポイント: `String` の行配列、`chars()` 走査、状態更新
- コード設計: 数式で一気に書く版と、状態を再現する版の比較

問題の本質:
- 各文字が「どの行に落ちるか」を正しく並べればよい
- 直接最終インデックスを計算するより、行番号と進行方向を状態として持つと壊れにくい
- ただし、周期 `2 * num_rows - 2` に気づくと、別の書き方もできる

解法候補:
1. 行バッファを持って simulation
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 本命。状態が分かりやすい
2. 周期を使って行ごとに文字を拾う
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 数式寄りで高速だが、やや考える量が増える
3. 行バッファ simulation の整理版
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 状態更新を関数へ分けて読みやすくする

採用解法の説明:
- `row` と `direction` を持ち、文字を 1 つ読むごとに対応する行へ積む
- 最上段に来たら下向き、最下段に来たら上向きへ切り替える
- こうすると、ジグザグに書く手順をそのまま再現できる
- `num_rows == 1` のときは周期が 0 になってしまうので先に返す

Rust観点の解説:
- `Vec<String>` を使うと、各行に `push` しながら最後に `concat()` できる
- `chars()` を使うのは、入力が Unicode を含んでも文字単位で安全だから
- direction は `isize` で持つと、上向き `-1` / 下向き `+1` を自然に表現できる

いいコード観点の解説:
- 本命実装は、状態が少なく、手続きと説明が一致している
- 周期版は賢いが、まず simulation を理解してから読む方が良い
- 整理版では、方向更新だけを関数に切り出して責務を分ける

落とし穴:
- `num_rows == 1` を処理せず、行番号更新で壊す
- 最上段と最下段で direction 切り替えのタイミングを誤る
- 周期版で中間行の斜め文字を取り漏らす
*/

struct Solution;
struct RowBufferSolution;
struct CycleSolution;
struct RefinedSolution;

impl RowBufferSolution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows <= 1 || s.len() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut row = 0usize;
        let mut going_down = true;

        for ch in s.chars() {
            rows[row].push(ch);
            if row == 0 {
                going_down = true;
            } else if row + 1 == num_rows {
                going_down = false;
            }
            row = if going_down { row + 1 } else { row - 1 };
        }

        rows.concat()
    }
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows <= 1 || s.len() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut row = 0usize;
        let mut direction = 1isize;

        for ch in s.chars() {
            rows[row].push(ch);

            if row == 0 {
                direction = 1;
            } else if row + 1 == num_rows {
                direction = -1;
            }

            row = (row as isize + direction) as usize;
        }

        rows.concat()
    }
}

impl CycleSolution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows <= 1 || s.len() <= num_rows {
            return s;
        }

        let chars: Vec<char> = s.chars().collect();
        let cycle = 2 * num_rows - 2;
        let mut result = String::with_capacity(chars.len());

        for row in 0..num_rows {
            let mut index = row;
            while index < chars.len() {
                result.push(chars[index]);

                let diagonal = index + cycle - 2 * row;
                if row != 0 && row + 1 != num_rows && diagonal < chars.len() {
                    result.push(chars[diagonal]);
                }

                index += cycle;
            }
        }

        result
    }
}

impl RefinedSolution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if num_rows <= 1 || s.len() <= num_rows {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let mut row = 0usize;
        let mut direction = 1isize;

        for ch in s.chars() {
            rows[row].push(ch);
            Self::advance(&mut row, &mut direction, num_rows);
        }

        rows.concat()
    }

    fn advance(row: &mut usize, direction: &mut isize, num_rows: usize) {
        if *row == 0 {
            *direction = 1;
        } else if *row + 1 == num_rows {
            *direction = -1;
        }

        *row = (*row as isize + *direction) as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(s: &str, num_rows: i32, expected: &str) {
        assert_eq!(RowBufferSolution::convert(s.to_string(), num_rows), expected);
        assert_eq!(Solution::convert(s.to_string(), num_rows), expected);
        assert_eq!(CycleSolution::convert(s.to_string(), num_rows), expected);
        assert_eq!(RefinedSolution::convert(s.to_string(), num_rows), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
    }

    #[test]
    fn single_row_is_identity() {
        assert_all_solutions("ABC", 1, "ABC");
    }

    #[test]
    fn four_rows_case() {
        assert_all_solutions("PAYPALISHIRING", 4, "PINALSIGYAHRPI");
    }

    #[test]
    fn short_string_is_unchanged() {
        assert_all_solutions("AB", 4, "AB");
    }
}
