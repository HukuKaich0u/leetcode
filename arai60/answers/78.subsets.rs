/*
この問題で鍛えること:
- アルゴリズム: subsets generation
- データ構造: 可変配列
- Rust での練習ポイント: iterative doubling, bit 演算, DFS
- コード設計: 「各要素を入れる/入れない」の 2 択を複数表現で捉える

問題の本質:
- 各要素について選択肢は 2 つだけ: 入れるか、入れないか
- したがって全部で `2^n` 個の部分集合がある
- この 2 択木を DFS でたどってもよいし、既存集合を倍増させてもよい

解法候補:
1. bitmask
   - 採用/非採用理由: 2^n の構造が直接見える
2. iterative doubling
   - 採用/非採用理由: 本命。実装が簡潔
3. DFS include/exclude
   - 採用/非採用理由: backtracking の基本形として有益

採用解法の説明:
- 最初は空集合だけ持つ
- 新しい要素 `x` を見るたび、既存の各部分集合へ `x` を追加したものを作る
- これで部分集合の個数が毎回 2 倍になる

Rust観点の解説:
- `out` を反復中に直接伸ばすと borrow 競合しやすいので、追加分を `next` へ集めてから extend する
- bitmask 版は `(mask >> bit) & 1` で選択を読む
- DFS 版は path push/pop の最小例として分かりやすい

いいコード観点の解説:
- 本命実装は「部分集合を倍増させる」という発想がそのままコードになっている
- 同じ問題でも bitmask/DFS/iterative で見え方が変わるので、表現変換の練習に向く
- 列挙問題では順序より完全性が重要で、テストも順不同で見るべき

落とし穴:
- `out` を走査しながら直接 push して無限に伸ばす
- bitmask のビット位置をずらす
- DFS の戻りがけで pop を忘れる
*/

struct Solution;
struct BitmaskSolution;
struct DfsSolution;

impl BitmaskSolution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = Vec::new();
        for mask in 0..(1usize << nums.len()) {
            let mut subset = Vec::new();
            for (bit, &value) in nums.iter().enumerate() {
                if (mask >> bit) & 1 == 1 {
                    subset.push(value);
                }
            }
            out.push(subset);
        }
        out
    }
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = vec![vec![]];
        for value in nums {
            let mut next = Vec::with_capacity(out.len());
            for subset in &out {
                let mut extended = subset.clone();
                extended.push(value);
                next.push(extended);
            }
            out.extend(next);
        }
        out
    }
}

impl DfsSolution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = Vec::new();
        let mut path = Vec::new();
        Self::dfs(0, &nums, &mut path, &mut out);
        out
    }

    fn dfs(index: usize, nums: &[i32], path: &mut Vec<i32>, out: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            out.push(path.clone());
            return;
        }
        Self::dfs(index + 1, nums, path, out);
        path.push(nums[index]);
        Self::dfs(index + 1, nums, path, out);
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut values: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for value in &mut values {
            value.sort_unstable();
        }
        values.sort();
        values
    }

    fn assert_all_solutions(nums: Vec<i32>, expected: Vec<Vec<i32>>) {
        let expected = normalize(expected);
        assert_eq!(normalize(BitmaskSolution::subsets(nums.clone())), expected);
        assert_eq!(normalize(Solution::subsets(nums.clone())), expected);
        assert_eq!(normalize(DfsSolution::subsets(nums)), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(
            vec![1, 2, 3],
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
        );
    }

    #[test]
    fn empty_input_case() {
        assert_all_solutions(vec![], vec![vec![]]);
    }
}
