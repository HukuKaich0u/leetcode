// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N * L) (Nがemailsの長さで、Lがその要素の文字列スライスの長さ)
  空間計算量:
*/

ues std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let unique_emails = HashSet::new();

        for email in emails {
            let Some((local_name, domain_name)) = email.split_once('@') else {
                eprintln!("There is no @ in this email address: {}", email);
                contine;
            };
            let normalized_local_name = String::new();

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
