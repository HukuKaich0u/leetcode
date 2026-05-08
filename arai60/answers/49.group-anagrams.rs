/*
この問題で鍛えること:
- アルゴリズム: grouping のための canonical key 設計
- データ構造: HashMap
- Rust での練習ポイント: 配列キー、`entry().or_default()`、結果順非依存テスト
- コード設計: sort キーと頻度キーの比較

問題の本質:
- anagram 判定を文字列同士の pairwise 比較でやるのではなく、同値類の代表キーへ正規化して集約する
- 同じ文字 multiset を持つ文字列は同じグループに入る
- したがって「どう正規化キーを作るか」が本体

解法候補:
1. 文字列ごとに sort した文字列をキーにする
   - 時間計算量: O(n * k log k)
   - 空間計算量: O(nk)
   - 採用/非採用理由: 実装が簡単で分かりやすい
2. 26 文字の頻度ベクトルをキーにする
   - 時間計算量: O(nk)
   - 空間計算量: O(nk)
   - 採用/非採用理由: 小文字英字という制約を使えて、本命
3. キー生成を helper に切り出す
   - 時間計算量: O(nk)
   - 空間計算量: O(nk)
   - 採用/非採用理由: 正規化処理の責務を分離できる

採用解法の説明:
- 各文字列について、26 文字の出現回数配列 `[u8; 26]` を作る
- anagram 同士はこの配列が一致するので、HashMap のキーにできる
- 各文字列をそのキーのバケットへ積めばグループ化できる

Rust観点の解説:
- `[u8; 26]` は `Eq + Hash` を持つので、そのまま `HashMap` のキーに使える
- `groups.entry(key).or_default().push(s)` で group 生成と push をまとめられる
- sort キー版では `bytes.sort_unstable()` のあと `String::from_utf8` で戻す

いいコード観点の解説:
- 頻度キー版は制約をうまく利用していて、比較コストの本質が見える
- helper 版を併記すると、「正規化」が独立した概念であることをコード上でも示せる
- group の順序は要件で本質ではないので、テスト側で normalize する

落とし穴:
- group の順序や group 内順序まで固定してしまう
- 文字列同士を毎回直接比較してしまう
- 制約が小文字英字であることを見落として汎用すぎる実装に寄せる
*/

use std::collections::HashMap;

struct Solution;
struct SortedKeySolution;
struct HelperKeySolution;

impl SortedKeySolution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();

        for string in strs {
            let mut bytes = string.as_bytes().to_vec();
            bytes.sort_unstable();
            let key = String::from_utf8(bytes).unwrap();
            groups.entry(key).or_default().push(string);
        }

        groups.into_values().collect()
    }
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for string in strs {
            let mut key = [0u8; 26];
            for byte in string.bytes() {
                key[(byte - b'a') as usize] += 1;
            }
            groups.entry(key).or_default().push(string);
        }

        groups.into_values().collect()
    }
}

impl HelperKeySolution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for string in strs {
            let key = Self::build_key(&string);
            groups.entry(key).or_default().push(string);
        }

        groups.into_values().collect()
    }

    fn build_key(string: &str) -> [u8; 26] {
        let mut key = [0u8; 26];
        for byte in string.bytes() {
            key[(byte - b'a') as usize] += 1;
        }
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut groups {
            group.sort();
        }
        groups.sort();
        groups
    }

    fn assert_all_solutions(input: Vec<String>, expected: Vec<Vec<String>>) {
        let expected = normalize(expected);
        assert_eq!(normalize(SortedKeySolution::group_anagrams(input.clone())), expected);
        assert_eq!(normalize(Solution::group_anagrams(input.clone())), expected);
        assert_eq!(normalize(HelperKeySolution::group_anagrams(input)), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec![
                vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
                vec!["bat".to_string()],
                vec!["nat".to_string(), "tan".to_string()],
            ],
        );
    }

    #[test]
    fn single_item_case() {
        assert_all_solutions(vec!["".to_string()], vec![vec!["".to_string()]]);
    }

    #[test]
    fn repeated_letters_case() {
        assert_all_solutions(
            vec!["abb", "bab", "bba", "xyz"]
                .into_iter()
                .map(String::from)
                .collect(),
            vec![
                vec!["abb".to_string(), "bab".to_string(), "bba".to_string()],
                vec!["xyz".to_string()],
            ],
        );
    }
}
