/*
この問題で鍛えること:
- アルゴリズム: backtracking, pruning
- データ構造: 可変文字列
- Rust での練習ポイント: `String` の push/pop、再帰 helper
- コード設計: 全列挙して後から捨てる実装と、不正枝を早めに切る実装の差

問題の本質:
- 長さ `2n` の文字列を全部作るのではなく、途中状態で「もう正解になれない枝」を切るのが本質
- 管理したい状態は `open` と `close` の残り個数だけ
- `close < open` になった瞬間、その枝は将来どう伸ばしても不正

解法候補:
1. 全列挙して後から妥当性判定
   - 時間計算量: O(2^(2n) * n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 比較用にはなるが無駄が大きい
2. 残り open/close を持つ backtracking
   - 時間計算量: 生成される正解数に比例
   - 空間計算量: O(n)
   - 採用/非採用理由: 本命
3. `left_used/right_used` で書く整理版
   - 採用/非採用理由: 状態の切り方の別表現として有益

採用解法の説明:
- `open > 0` なら `(` を置ける
- `close > open` なら `)` を置ける。これは「今まで置いた `(` を超えて閉じない」条件そのもの
- `open == 0 && close == 0` になったら 1 解完成

Rust観点の解説:
- `String` を 1 本使い回し、push/pop で戻すと余計なコピーを抑えられる
- helper へ `&mut String` と `&mut Vec<String>` を渡すのが自然
- 別解で `Vec<char>` を使うと、文字列構築の別表現も確認できる

いいコード観点の解説:
- 列挙問題は「何を状態に持つか」がコードの読みやすさを決める
- 本命実装は制約がそのまま if 条件になっていて、問題文との対応が強い
- 素朴実装を併記すると pruning の価値が具体的に伝わる

落とし穴:
- `close > open` の条件を逆にして不正枝を伸ばす
- 完成判定を `path.len() == 2 * n` だけに依存して条件を崩す
- `String` を毎回 clone して無駄を増やす
*/

struct Solution;
struct NaiveSolution;
struct HelperSolution;

impl NaiveSolution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(pos: usize, n: usize, path: &mut Vec<char>, out: &mut Vec<String>) {
            if pos == 2 * n {
                if is_valid(path) {
                    out.push(path.iter().collect());
                }
                return;
            }
            path.push('(');
            dfs(pos + 1, n, path, out);
            path.pop();

            path.push(')');
            dfs(pos + 1, n, path, out);
            path.pop();
        }

        fn is_valid(path: &[char]) -> bool {
            let mut balance = 0;
            for &ch in path {
                if ch == '(' {
                    balance += 1;
                } else {
                    balance -= 1;
                }
                if balance < 0 {
                    return false;
                }
            }
            balance == 0
        }

        let mut out = Vec::new();
        let mut path = Vec::new();
        dfs(0, n as usize, &mut path, &mut out);
        out
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(open: i32, close: i32, path: &mut String, out: &mut Vec<String>) {
            if open == 0 && close == 0 {
                out.push(path.clone());
                return;
            }

            if open > 0 {
                path.push('(');
                dfs(open - 1, close, path, out);
                path.pop();
            }
            if close > open {
                path.push(')');
                dfs(open, close - 1, path, out);
                path.pop();
            }
        }

        let mut out = Vec::new();
        let mut path = String::new();
        dfs(n, n, &mut path, &mut out);
        out
    }
}

impl HelperSolution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut out = Vec::new();
        let mut path = Vec::with_capacity((2 * n) as usize);
        Self::dfs(0, 0, n, &mut path, &mut out);
        out
    }

    fn dfs(left_used: i32, right_used: i32, n: i32, path: &mut Vec<char>, out: &mut Vec<String>) {
        if left_used == n && right_used == n {
            out.push(path.iter().collect());
            return;
        }
        if left_used < n {
            path.push('(');
            Self::dfs(left_used + 1, right_used, n, path, out);
            path.pop();
        }
        if right_used < left_used {
            path.push(')');
            Self::dfs(left_used, right_used + 1, n, path, out);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut values: Vec<String>) -> Vec<String> {
        values.sort();
        values
    }

    fn assert_all_solutions(n: i32, expected: &[&str]) {
        let expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        assert_eq!(normalize(NaiveSolution::generate_parenthesis(n)), expected);
        assert_eq!(normalize(Solution::generate_parenthesis(n)), expected);
        assert_eq!(normalize(HelperSolution::generate_parenthesis(n)), expected);
    }

    #[test]
    fn n_three_case() {
        assert_all_solutions(3, &["((()))", "(()())", "(())()", "()(())", "()()()"]);
    }

    #[test]
    fn n_one_case() {
        assert_all_solutions(1, &["()"]);
    }
}
