/*
この問題で鍛えること:
- アルゴリズム: BFS と最短路
- データ構造: HashSet, HashMap, VecDeque
- Rust での練習ポイント: queue ベース探索、文字列の近傍生成、補助関数への切り出し
- コード設計: 素直な BFS、本命の 26 文字置換、発展的な双方向 BFS の比較

問題の本質:
- 1 回の文字変換を 1 辺とみなすと、問題は無向 unweighted graph の最短路になる
- 欲しいのは最小変換回数なので、重みなし最短路の標準解法は BFS
- 文字列問題に見えても、本質は「どう隣接ノードを表現するか」の graph 問題

解法候補:
1. wildcard graph を明示的に作って BFS
   - 時間計算量: 前処理 O(nL)、探索は辺数依存
   - 空間計算量: O(nL)
   - 採用/非採用理由: 近傍生成を前処理へ寄せられる。比較対象として有益
2. 毎回 26 文字置換で近傍を作る BFS
   - 時間計算量: O(n * L * 26) 近辺
   - 空間計算量: O(n)
   - 採用/非採用理由: 面接でも説明しやすく、本命
3. 双方向 BFS
   - 時間計算量: 実運用では探索幅をかなり減らせる
   - 空間計算量: O(n)
   - 採用/非採用理由: 発展的だが、探索の考え方を広げるのに良い

採用解法の説明:
- 単語集合を `HashSet` に入れ、未訪問辞書として使う
- queue から 1 単語ずつ取り出し、各文字位置について `a..z` の 26 通りへ置換した近傍を作る
- 辞書に存在する単語だけを次層へ積み、同時に辞書から削除して訪問済みにする
- BFS なので、最初に `end_word` へ到達したときの深さが最短距離になる

Rust観点の解説:
- queue は `VecDeque` が自然
- 近傍生成では `Vec<char>` にして 1 文字ずつ差し替えると分かりやすい
- `dict.remove(&next)` を訪問判定と訪問済みマークの両方に使うと、visited 用の別 set を減らせる
- 整理版として `neighbors` を helper に切ると、本体は BFS の骨格だけ読める

いいコード観点の解説:
- 本命実装は「graph をどう表現するか」より「BFS をどう進めるか」が前面に出る
- wildcard graph 版は、近傍生成の責務を前処理へ移した設計比較になる
- 双方向 BFS 版は速いが、frontier の入れ替えと辞書更新で少し複雑になる

落とし穴:
- `end_word` が辞書に無いのに探索を始める
- queue に積む前ではなく、取り出した後に visited にすると同じ単語を何度も積んでしまう
- 文字を差し替えたあとに元へ戻さず、次の位置の探索を壊す
- BFS なのに DFS 的な再帰へ流れて最短距離保証を失う
*/

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;
struct WildcardGraphSolution;
struct BidirectionalBfsSolution;
struct RefinedSolution;

impl WildcardGraphSolution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut words = word_list;
        if !words.iter().any(|word| word == &end_word) {
            return 0;
        }
        if !words.iter().any(|word| word == &begin_word) {
            words.push(begin_word.clone());
        }

        let begin_index = words.iter().position(|word| word == &begin_word).unwrap();
        let end_index = words.iter().position(|word| word == &end_word).unwrap();

        let mut patterns: HashMap<String, Vec<usize>> = HashMap::new();
        for (index, word) in words.iter().enumerate() {
            for pattern in Self::patterns(word) {
                patterns.entry(pattern).or_default().push(index);
            }
        }

        let mut visited = vec![false; words.len()];
        let mut queue = VecDeque::from([(begin_index, 1)]);
        visited[begin_index] = true;
        let mut used_patterns = HashSet::new();

        while let Some((index, steps)) = queue.pop_front() {
            if index == end_index {
                return steps;
            }

            for pattern in Self::patterns(&words[index]) {
                if !used_patterns.insert(pattern.clone()) {
                    continue;
                }
                if let Some(neighbors) = patterns.get(&pattern) {
                    for &next_index in neighbors {
                        if !visited[next_index] {
                            visited[next_index] = true;
                            queue.push_back((next_index, steps + 1));
                        }
                    }
                }
            }
        }

        0
    }

    fn patterns(word: &str) -> Vec<String> {
        let chars: Vec<char> = word.chars().collect();
        let mut patterns = Vec::with_capacity(chars.len());

        for index in 0..chars.len() {
            let mut pattern = chars.clone();
            pattern[index] = '*';
            patterns.push(pattern.iter().collect());
        }

        patterns
    }
}

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<String> = word_list.into_iter().collect();
        if !dict.contains(&end_word) {
            return 0;
        }

        let mut queue = VecDeque::from([(begin_word.clone(), 1)]);
        dict.remove(&begin_word);

        while let Some((word, steps)) = queue.pop_front() {
            if word == end_word {
                return steps;
            }

            let mut chars: Vec<char> = word.chars().collect();
            for index in 0..chars.len() {
                let original = chars[index];
                for byte in b'a'..=b'z' {
                    let candidate = byte as char;
                    if candidate == original {
                        continue;
                    }

                    chars[index] = candidate;
                    let next: String = chars.iter().collect();
                    if dict.remove(&next) {
                        queue.push_back((next, steps + 1));
                    }
                }
                chars[index] = original;
            }
        }

        0
    }
}

impl BidirectionalBfsSolution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<String> = word_list.into_iter().collect();
        if !dict.contains(&end_word) {
            return 0;
        }

        let mut front = HashSet::from([begin_word.clone()]);
        let mut back = HashSet::from([end_word.clone()]);
        dict.remove(&begin_word);
        dict.remove(&end_word);

        let mut steps = 1;

        while !front.is_empty() && !back.is_empty() {
            if front.len() > back.len() {
                std::mem::swap(&mut front, &mut back);
            }

            let mut next_front = HashSet::new();
            for word in &front {
                for neighbor in RefinedSolution::neighbors(word) {
                    if back.contains(&neighbor) {
                        return steps + 1;
                    }
                    if dict.remove(&neighbor) {
                        next_front.insert(neighbor);
                    }
                }
            }

            front = next_front;
            steps += 1;
        }

        0
    }
}

impl RefinedSolution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut dict: HashSet<String> = word_list.into_iter().collect();
        if !dict.contains(&end_word) {
            return 0;
        }

        let mut queue = VecDeque::from([(begin_word.clone(), 1)]);
        dict.remove(&begin_word);

        while let Some((word, steps)) = queue.pop_front() {
            if word == end_word {
                return steps;
            }

            for neighbor in Self::neighbors(&word) {
                if dict.remove(&neighbor) {
                    queue.push_back((neighbor, steps + 1));
                }
            }
        }

        0
    }

    fn neighbors(word: &str) -> Vec<String> {
        let mut chars: Vec<char> = word.chars().collect();
        let mut result = Vec::new();

        for index in 0..chars.len() {
            let original = chars[index];
            for byte in b'a'..=b'z' {
                let candidate = byte as char;
                if candidate == original {
                    continue;
                }
                chars[index] = candidate;
                result.push(chars.iter().collect());
            }
            chars[index] = original;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(
        begin_word: &str,
        end_word: &str,
        word_list: &[&str],
        expected: i32,
    ) {
        let words: Vec<String> = word_list.iter().map(|word| (*word).to_string()).collect();

        assert_eq!(
            WildcardGraphSolution::ladder_length(
                begin_word.to_string(),
                end_word.to_string(),
                words.clone(),
            ),
            expected
        );
        assert_eq!(
            Solution::ladder_length(begin_word.to_string(), end_word.to_string(), words.clone()),
            expected
        );
        assert_eq!(
            BidirectionalBfsSolution::ladder_length(
                begin_word.to_string(),
                end_word.to_string(),
                words.clone(),
            ),
            expected
        );
        assert_eq!(
            RefinedSolution::ladder_length(begin_word.to_string(), end_word.to_string(), words),
            expected
        );
    }

    #[test]
    fn basic_case() {
        assert_all_solutions("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"], 5);
    }

    #[test]
    fn missing_end_word_returns_zero() {
        assert_all_solutions("hit", "cog", &["hot", "dot", "dog", "lot", "log"], 0);
    }

    #[test]
    fn unreachable_case_returns_zero() {
        assert_all_solutions("aaa", "bbb", &["aab", "abb", "bab"], 0);
    }
}
