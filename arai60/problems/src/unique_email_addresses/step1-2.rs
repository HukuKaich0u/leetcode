use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();

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

            unique_emails.insert(format!("{}@{}", normalized_local, domain));
        }

        unique_emails.len() as i32
    }
}
