/*
この問題で鍛えること:
- アルゴリズム: 正規化してから比較する発想
- データ構造: HashSet
- Rust での練習ポイント: `split_once`、`split`、`replace`、補助関数化
- コード設計: equality 判定を直接複雑化せず、canonical form を作る感覚

問題の本質:
- 比較したいのは元の email 文字列ではなく、仕様に従って正規化した email
- local part では `.` を無視し、`+` 以降を捨てる。一方 domain はそのまま
- つまり問題は文字列比較ではなく、「canonical form を 1 つ作って set に入れる」問題

解法候補:
1. 毎回全メール同士を仕様付きで比較する
   - 時間計算量: O(n^2 * L)
   - 空間計算量: O(1)
   - 採用/非採用理由: 正規化の考え方を使えていない
2. 文字列を正規化して HashSet へ入れる
   - 時間計算量: O(nL)
   - 空間計算量: O(nL)
   - 採用/非採用理由: 要件に最も自然で、本命
3. 正規化処理を helper へ切り出す
   - 時間計算量: O(nL)
   - 空間計算量: O(nL)
   - 採用/非採用理由: 処理の責務が見えやすい

採用解法の説明:
- email を `local` と `domain` に分ける
- `local` は `+` より左だけ取り、`.` を除去する
- 正規化後の `local@domain` を HashSet へ入れる
- 最後に set のサイズが答えになる

Rust観点の解説:
- `split_once('@')` で local と domain を一度に分けられる
- `split('+').next().unwrap()` で `+` より左だけを取れる
- helper 版では `normalize_email` を切り出し、正規化ルールを 1 か所へ集められる

いいコード観点の解説:
- 仕様を「正規化関数」として閉じ込めると、集合への登録側がきれいになる
- 本命実装は標準 API を素直につないで書ける
- helper 版は再利用性とテストしやすさが高い

落とし穴:
- domain 側の `.` まで削ってしまう
- `+` より後ろを消し忘れる
- 正規化前の email を set に入れてしまう
*/

use std::collections::HashSet;

struct Solution;
struct NormalizationSolution;
struct HelperSolution;

impl NormalizationSolution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut seen = HashSet::new();

        for email in emails {
            let (local, domain) = email.split_once('@').unwrap();
            let local = local
                .chars()
                .take_while(|&ch| ch != '+')
                .filter(|&ch| ch != '.')
                .collect::<String>();
            seen.insert(format!("{local}@{domain}"));
        }

        seen.len() as i32
    }
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut seen = HashSet::new();

        for email in emails {
            let (local, domain) = email.split_once('@').unwrap();
            let local = local.split('+').next().unwrap().replace('.', "");
            seen.insert(format!("{local}@{domain}"));
        }

        seen.len() as i32
    }
}

impl HelperSolution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut seen = HashSet::new();

        for email in emails {
            seen.insert(Self::normalize_email(&email));
        }

        seen.len() as i32
    }

    fn normalize_email(email: &str) -> String {
        let (local, domain) = email.split_once('@').unwrap();
        let local = local.split('+').next().unwrap().replace('.', "");
        format!("{local}@{domain}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(emails: Vec<String>, expected: i32) {
        assert_eq!(NormalizationSolution::num_unique_emails(emails.clone()), expected);
        assert_eq!(Solution::num_unique_emails(emails.clone()), expected);
        assert_eq!(HelperSolution::num_unique_emails(emails), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(
            vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string(),
            ],
            2,
        );
    }

    #[test]
    fn dots_and_plus_are_normalized() {
        assert_all_solutions(
            vec!["a.b+c@x.com".to_string(), "ab@x.com".to_string()],
            1,
        );
    }

    #[test]
    fn different_domains_remain_distinct() {
        assert_all_solutions(vec!["a@x.com".to_string(), "a@y.com".to_string()], 2);
    }
}
