/*
この問題で鍛えること:
- アルゴリズム: 2 次元座標変換の分解
- データ構造: 補助行列あり/なしの比較
- Rust での練習ポイント: 2 次元 `Vec` の in-place 更新、transpose、layer 走査
- コード設計: 直接 4 点回すより、意味のある操作へ分解する感覚

問題の本質:
- 90 度回転を 1 発の添字式で考えると壊れやすい
- しかし「転置」と「各行の反転」に分けると、各操作の意味が明確になる
- 問題は座標変換を暗算することではなく、変換を安全な基本操作に分解すること

解法候補:
1. 補助行列を作る
   - 時間計算量: O(n^2)
   - 空間計算量: O(n^2)
   - 採用/非採用理由: 一番分かりやすいが in-place 条件を使えていない
2. 転置して各行を反転
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命。説明しやすく壊れにくい
3. layer ごとに 4 点を回す
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 補助配列無しで直接回せるが、添字事故が起きやすい

採用解法の説明:
- 転置すると、`(row, col)` の要素は `(col, row)` へ移る
- その後で各行を reverse すると、`(col, row)` が `(col, n - 1 - row)` へ移り、90 度時計回り回転と一致する
- つまり複雑な回転を、`transpose` と `reverse` という理解しやすい 2 操作に分解できる

Rust観点の解説:
- 転置は `for row in 0..n { for col in row + 1..n { ... } }` と書けば、対角線より上だけを swap すればよい
- `row.reverse()` で行単位の反転を簡潔に書ける
- layer 版は添字が多いので、補助変数に名前を付けることが重要

いいコード観点の解説:
- 本命実装は、何をしているかをコード構造がそのまま表している
- 補助行列版は理解の入口としては優秀なので、比較対象として価値がある
- layer 版は低レベルだが、座標変換を自前で回す力を鍛えられる

落とし穴:
- 転置で `col` を 0 から回して二重 swap してしまう
- 行反転ではなく列反転をしてしまう
- layer 版で offset 計算を 1 つずらして壊す
*/

struct Solution;
struct BufferSolution;
struct LayerRotationSolution;
struct RefinedSolution;

impl BufferSolution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let mut rotated = vec![vec![0; n]; n];

        for row in 0..n {
            for col in 0..n {
                rotated[col][n - 1 - row] = matrix[row][col];
            }
        }

        *matrix = rotated;
    }
}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for row in 0..n {
            for col in row + 1..n {
                let temp = matrix[row][col];
                matrix[row][col] = matrix[col][row];
                matrix[col][row] = temp;
            }
        }

        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

impl LayerRotationSolution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for layer in 0..n / 2 {
            let last = n - 1 - layer;
            for col in layer..last {
                let offset = col - layer;
                let top = matrix[layer][col];

                matrix[layer][col] = matrix[last - offset][layer];
                matrix[last - offset][layer] = matrix[last][last - offset];
                matrix[last][last - offset] = matrix[col][last];
                matrix[col][last] = top;
            }
        }
    }
}

impl RefinedSolution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::transpose(matrix);
        Self::reverse_rows(matrix);
    }

    fn transpose(matrix: &mut [Vec<i32>]) {
        let n = matrix.len();
        for row in 0..n {
            for col in row + 1..n {
                let temp = matrix[row][col];
                matrix[row][col] = matrix[col][row];
                matrix[col][row] = temp;
            }
        }
    }

    fn reverse_rows(matrix: &mut [Vec<i32>]) {
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(input: Vec<Vec<i32>>, expected: Vec<Vec<i32>>) {
        let mut buffer = input.clone();
        BufferSolution::rotate(&mut buffer);
        assert_eq!(buffer, expected);

        let mut main = input.clone();
        Solution::rotate(&mut main);
        assert_eq!(main, expected);

        let mut layer = input.clone();
        LayerRotationSolution::rotate(&mut layer);
        assert_eq!(layer, expected);

        let mut refined = input;
        RefinedSolution::rotate(&mut refined);
        assert_eq!(refined, expected);
    }

    #[test]
    fn handles_one_by_one() {
        assert_all_solutions(vec![vec![1]], vec![vec![1]]);
    }

    #[test]
    fn handles_two_by_two() {
        assert_all_solutions(vec![vec![1, 2], vec![3, 4]], vec![vec![3, 1], vec![4, 2]]);
    }

    #[test]
    fn handles_three_by_three() {
        assert_all_solutions(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
        );
    }
}
