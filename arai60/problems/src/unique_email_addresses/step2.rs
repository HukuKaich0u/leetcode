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
  - 計算量がちょっと大きいかも(?)

  他の人のコードを読んで考えたこと
  - これを前からそのまま書こうとするのはよろしくなく、実現したいロジックを最小で
実装していくことを繰り返して、そこからやっと１つの関数内の少し後ろで使うような変数を
最初に明示的に定義する、整ったコードが実現できているので、写経する時には、コードを
追うのではなく、思考を追うようにこれからも心がけたい

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 変数やループのインデックスの命名は問題文中で与えられたものを極力使う。なぜなら
その方がコードを読むときにコンテキストを共有しやすくなるから。ただし、長すぎても
可読性が低下するのでナンセンスで、せいぜい2こ区切りができる程度、つまり3単語くらい。

  - unwrap()だと@がないemailが入ってきた時点で落ちるので、全部のemail addressに対して
処理ができなくなり、不正な値が入ってきたら問題の回答を満たせなくなるので、別のハンドリングをしよう

  - matchの処理で、ワイルドカードを使うよりも、アーム内でcharacterを使うんだから、
otherでマッピングしてそのまま使った方が自然
*/

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();

        for email in emails {
            let Some((local_name, domain_name)) = email.split_once('@') else {
                eprintln!("There is no @ in this email address: {}", email);
                continue;
            };
            let mut normalized_local_name = String::new();

            for character in local_name.chars() {
                match character {
                    '.' => {},
                    '+' => break,
                    other => normalized_local_name.push(other),
                }
            }

            unique_emails.insert(format!("{}@{}", normalized_local_name, domain_name));
        }

        unique_emails.len() as i32
    }
}
