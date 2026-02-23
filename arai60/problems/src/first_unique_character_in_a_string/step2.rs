// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  - 最小限の実装だとこれ

  他の人のコードを読んで考えたこと
  - Iwasaさんとt9a-devさんのコードはちょっと難しすぎた。
  - 不正な値に対する処理とか....

  - Rustらしさを求めるのであれば、文字列周りにはちゃんと強くならないといけないな。

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 命名は一応それだけで意味が取れるようにした
  - indexとかcharacterとかは流石に慣習的にi, cである程度共通認識だと思うんだけど...

  - 配列の型の宣言はturbofishに統一する

  - 2個目のforは、chars.into_iter()にして、マッピングを(index, character)にしても
いいんだけど、ムーブするかどうか次第なので、今回のケースではどちらでも良い
*/

use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = HashMap::<char, i32>::new();
        let chars = s.chars().collect::<Vec<char>>();

        for &character in &chars {
            *count.entry(character).or_insert(0) += 1;
        }

        for (index, &character) in chars.iter().enumerate() {
            if count.get(&character) == Some(&1) {
                return index as i32;
            }
        }

        -1
    }
}
