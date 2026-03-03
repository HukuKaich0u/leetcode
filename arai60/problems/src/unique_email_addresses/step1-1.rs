/// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 時間内に考えていたことをコードに落とせなかった
  - 文字列スライスを解析する処理

  何を考えて解いていたか
  - 与えられた配列を回して要素である文字スライスを取り出し、解析する
  - まずは、文字スライスを@の前後で切り離して、ハッシュマップにしまう。
  - local nameの方には、"."を取り除いて、"+"から@までは無視する処理をして
前後の文字列スライスをくっつける処理を追加する
  - domain namesの方は、切り出すだけで良い
  - local nameとdomain nameのペアをHashSetに格納することで、重複を除ける
  - 最終は、そのHashSetの長さを返す

  想定ユースケース
  -

  正解してから気づいたこと
  - 自分の回答の方針は逆に複雑にしようとしていた、というのも文字列スライスを解析
したあとに、そこからどうやって唯一のemail addressを表現するかで、HashSetにさらに
tupleを入れるとか、@の前後の文字列スライスをペアにしてHashSetに格納するって考えて
いたけど、普通に前後でまたくっつけてHashSetに格納すれば、データが複雑にならないし、
唯一のemail addressを表現していることが明確になる

  - 考えていたことには、自分で@の前後で文字列スライスを結合するって書いてるのに、
頭の中ではHashSetに@の前後でtupleに入れて::HashSet<(&str, &str)> にしようとしていた
のが、まだ頭がごちゃついてる証拠だなと感じた

  - つまり、逆に自分で勝手に複雑にしようとしていた
*/

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique = HashSet::new();

        for email in emails {
            let (local, domain) = email.split_once('@').unwrap();

            let mut normalized_local = String::new();
            for c in local.chars() {
                match c {
                    '.' => {},
                    '+' => break,
                    _ => normalized_local.push(c),
                }
            }

            unique.insert(format!("{normalized_local}@{domain}"));
        }

        unique.len() as i32
    }

}

