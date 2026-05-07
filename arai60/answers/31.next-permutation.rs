/*
この問題で鍛えること:
- アルゴリズム: 辞書順で次を作るための greedy
- データ構造: suffix の性質を使った in-place 更新
- Rust での練習ポイント: 逆順走査、範囲 reverse、`Option<usize>` の扱い
- コード設計: 全列挙の素朴さから、本質的な pivot 操作へ落とす感覚

問題の本質:
- 次の permutation を作るには、できるだけ右側だけを最小限変更したい
- 右から見て降順になっている suffix は、すでにその並びで取りうる最大値
- だから最初に上昇が見つかる位置 `pivot` を少しだけ大きくし、その右側を最小化すれば次の辞書順になる

解法候補:
1. 全 permutation を列挙して次を探す
   - 時間計算量: 実用外
   - 空間計算量: 大きい
   - 採用/非採用理由: 定義確認にはなるが、問題の本質を使えていない
2. `pivot` を探して swap + reverse
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: suffix の性質を最も素直に使えて、本命
3. 1 層ずつ rotation 的に考える版
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: in-place ではあるが、本命より説明が長い

採用解法の説明:
- 右から見て初めて `nums[i] < nums[i + 1]` になる位置が `pivot`
- `pivot` より右は降順なので、その中で `nums[pivot]` より大きい最右要素と swap すれば、増加量を最小にできる
- swap 後も suffix は降順のままなので、そこを reverse すれば最小の昇順になる
- `pivot` が無ければ配列全体が降順なので、全体を reverse して最小の permutation に戻る

Rust観点の解説:
- `for i in (0..nums.len() - 1).rev()` で右からの探索を素直に書ける
- `nums[pivot + 1..].reverse()` のように suffix をそのまま反転できる
- 整理版では `find_pivot` と `find_swap_candidate` を分けて、探索の意図を明示できる

いいコード観点の解説:
- `pivot` はこの問題固有の重要概念なので、そのまま変数名にする
- 本命実装は 1 メソッドで全体像が見えるのが強み
- 一方で整理版は、「どこを探しているか」を関数名に出せるので、理解の補助になる

落とし穴:
- `pivot` より右で最初の大きい要素ではなく、最右の大きい要素を選ぶ必要がある
- suffix が降順だから sort ではなく reverse で十分なことを見落とす
- 全体降順ケースで reverse を忘れる
*/

struct Solution;
struct BruteForceSolution;
struct LayerSwapSolution;
struct RefinedSolution;

impl BruteForceSolution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let original = nums.clone();
        let mut permutations = Self::generate_permutations(nums.clone());
        permutations.sort();
        permutations.dedup();

        let position = permutations
            .iter()
            .position(|candidate| candidate == &original)
            .expect("original permutation must exist");
        let next = permutations[(position + 1) % permutations.len()].clone();
        *nums = next;
    }

    fn generate_permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut result = Vec::new();
        Self::permute(0, &mut nums, &mut result);
        result
    }

    fn permute(start: usize, nums: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            result.push(nums.clone());
            return;
        }

        for index in start..nums.len() {
            nums.swap(start, index);
            Self::permute(start + 1, nums, result);
            nums.swap(start, index);
        }
    }
}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut pivot = None;
        for index in (0..nums.len() - 1).rev() {
            if nums[index] < nums[index + 1] {
                pivot = Some(index);
                break;
            }
        }

        if let Some(pivot_index) = pivot {
            for index in (pivot_index + 1..nums.len()).rev() {
                if nums[index] > nums[pivot_index] {
                    nums.swap(pivot_index, index);
                    break;
                }
            }
            nums[pivot_index + 1..].reverse();
        } else {
            nums.reverse();
        }
    }
}

impl LayerSwapSolution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut pivot_index = nums.len() - 1;
        while pivot_index > 0 && nums[pivot_index - 1] >= nums[pivot_index] {
            pivot_index -= 1;
        }

        if pivot_index == 0 {
            nums.reverse();
            return;
        }

        let pivot = pivot_index - 1;
        let mut swap_index = nums.len() - 1;
        while nums[swap_index] <= nums[pivot] {
            swap_index -= 1;
        }

        nums.swap(pivot, swap_index);
        nums[pivot + 1..].reverse();
    }
}

impl RefinedSolution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        if let Some(pivot) = Self::find_pivot(nums) {
            let swap_candidate = Self::find_swap_candidate(nums, pivot);
            nums.swap(pivot, swap_candidate);
            nums[pivot + 1..].reverse();
        } else {
            nums.reverse();
        }
    }

    fn find_pivot(nums: &[i32]) -> Option<usize> {
        (0..nums.len() - 1).rev().find(|&index| nums[index] < nums[index + 1])
    }

    fn find_swap_candidate(nums: &[i32], pivot: usize) -> usize {
        (pivot + 1..nums.len())
            .rev()
            .find(|&index| nums[index] > nums[pivot])
            .expect("pivot guarantees a larger value on the right")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(input: Vec<i32>, expected: Vec<i32>) {
        let mut brute_force = input.clone();
        BruteForceSolution::next_permutation(&mut brute_force);
        assert_eq!(brute_force, expected);

        let mut main = input.clone();
        Solution::next_permutation(&mut main);
        assert_eq!(main, expected);

        let mut layer_swap = input.clone();
        LayerSwapSolution::next_permutation(&mut layer_swap);
        assert_eq!(layer_swap, expected);

        let mut refined = input;
        RefinedSolution::next_permutation(&mut refined);
        assert_eq!(refined, expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(vec![1, 2, 3], vec![1, 3, 2]);
    }

    #[test]
    fn descending_case_wraps_around() {
        assert_all_solutions(vec![3, 2, 1], vec![1, 2, 3]);
    }

    #[test]
    fn duplicate_values_are_handled() {
        assert_all_solutions(vec![1, 1, 5], vec![1, 5, 1]);
    }

    #[test]
    fn pivot_in_the_middle() {
        assert_all_solutions(vec![1, 3, 2], vec![2, 1, 3]);
    }
}
