/*
この問題で鍛えること:
- アルゴリズム: in-place 配列更新での two pointers
- データ構造: 追加配列を使う版と使わない版の比較
- Rust での練習ポイント: `&mut Vec<i32>`、添字更新、破壊的変更のテスト
- コード設計: 素朴に別配列へ逃がす版から、in-place の本命実装へ寄せる流れ

問題の本質:
- 要求は「0 を後ろへ寄せる」ことではなく、「非 0 を元の順序のまま前へ詰める」こと
- stable に詰めるので、左から順に「次に非 0 を置く場所」を管理できれば十分
- つまり、配列全体を都度並べ替える問題ではなく、書き込み位置を育てる問題として見る

解法候補:
1. 別配列へ非 0 を集めて、残りを 0 で埋める
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - データ構造: 補助 `Vec<i32>`
   - 採用/非採用理由: 一番分かりやすいが in-place 条件を使い切れていない
2. write pointer で上書きして、最後に 0 を埋める
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - データ構造: 添字 2 本
   - 採用/非採用理由: 不変条件が説明しやすく、本命
3. swap ベースで非 0 を前へ寄せる
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - データ構造: 添字 2 本
   - 採用/非採用理由: 実装は短いが、不要な swap が起きることがあり、本命より少し説明が難しい

採用解法の説明:
- `write` を「次に非 0 を置くべき index」とする
- 左から `read` を進め、非 0 を見つけたら `nums[write]` へ書いて `write += 1`
- この時点で、`[0, write)` は「元の順序を保った非 0 の確定領域」になる
- 走査が終わったら、残り `[write, n)` を 0 で埋めれば完成
- 不変条件は「`write` より左には、ここまでに見た非 0 が順序を保って並んでいる」こと

Rust観点の解説:
- `&mut Vec<i32>` を直接更新するので、返り値ではなく副作用をテストする
- 添字アクセスで済むので、可変参照を複数同時に持つ必要がなく、借用が単純
- `nums[write] = nums[read]` は copy 型 `i32` だから素直に書ける

いいコード観点の解説:
- 本命実装では `write` の意味を固定し、役割を 1 つにする
- `fill_tail_with_zeroes` を切り出す整理版は、「詰める処理」と「余りを埋める処理」を分けて読める
- swap 版は短いが、なぜ stable になるかが少し見えづらいので比較材料に回す

落とし穴:
- 0 を見つけるたびに後ろへ送ろうとして、相対順を壊す
- in-place にこだわりすぎて、`write` が何を意味するか曖昧なまま進める
- 上書き版で、最後の 0 埋めを忘れる
- swap 版で `read == write` のときも毎回気にしすぎてロジックを複雑にする
*/

struct Solution;
struct BufferSolution;
struct SwapSolution;
struct RefinedSolution;

impl BufferSolution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut non_zeroes = Vec::with_capacity(nums.len());

        for &value in nums.iter() {
            if value != 0 {
                non_zeroes.push(value);
            }
        }

        let non_zero_count = non_zeroes.len();
        nums[..non_zero_count].copy_from_slice(&non_zeroes);
        for value in nums.iter_mut().skip(non_zero_count) {
            *value = 0;
        }
    }
}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // `write` は、次に非 0 を置く位置を指す。
        let mut write = 0usize;

        for read in 0..nums.len() {
            if nums[read] != 0 {
                nums[write] = nums[read];
                write += 1;
            }
        }

        // 非 0 の確定領域より後ろは、すべて 0 で埋める。
        for value in nums.iter_mut().skip(write) {
            *value = 0;
        }
    }
}

impl SwapSolution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut write = 0usize;

        for read in 0..nums.len() {
            if nums[read] != 0 {
                nums.swap(write, read);
                write += 1;
            }
        }
    }
}

impl RefinedSolution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let write = Self::compact_non_zeroes(nums);
        Self::fill_tail_with_zeroes(nums, write);
    }

    fn compact_non_zeroes(nums: &mut [i32]) -> usize {
        let mut write = 0usize;

        for read in 0..nums.len() {
            if nums[read] != 0 {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write
    }

    fn fill_tail_with_zeroes(nums: &mut [i32], start: usize) {
        for value in nums.iter_mut().skip(start) {
            *value = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(input: Vec<i32>, expected: Vec<i32>) {
        let mut buffer = input.clone();
        BufferSolution::move_zeroes(&mut buffer);
        assert_eq!(buffer, expected);

        let mut main = input.clone();
        Solution::move_zeroes(&mut main);
        assert_eq!(main, expected);

        let mut swap = input.clone();
        SwapSolution::move_zeroes(&mut swap);
        assert_eq!(swap, expected);

        let mut refined = input;
        RefinedSolution::move_zeroes(&mut refined);
        assert_eq!(refined, expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn keeps_relative_order_of_non_zero_values() {
        assert_all_solutions(vec![4, 0, 5, 0, 0, 3], vec![4, 5, 3, 0, 0, 0]);
    }

    #[test]
    fn handles_all_zeroes() {
        assert_all_solutions(vec![0, 0, 0], vec![0, 0, 0]);
    }

    #[test]
    fn handles_zeroes_at_the_end() {
        assert_all_solutions(vec![1, 2, 3, 0, 0], vec![1, 2, 3, 0, 0]);
    }
}
