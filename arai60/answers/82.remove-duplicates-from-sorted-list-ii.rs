/*
この問題で鍛えること:
- アルゴリズム: run 単位の削除
- データ構造: sorted linked list
- Rust での練習ポイント: dummy node、`take()` を使った連結の付け替え
- コード設計: 「1 個残す」問題との違いを実装へ落とす力

問題の本質:
- sorted なので、重複値は必ず連続する
- この問題では重複値を 1 個残すのではなく、同じ値の run 全体を捨てる
- したがって「今の run は長さ 1 か、それとも 2 以上か」を見分けるのが核心

解法候補:
1. 配列化して個数を数え、1 回だけ出た値で作り直す
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: わかりやすいが list 更新の練習にならない
2. dummy node を使って run ごとに削除する
   - 時間計算量: O(n)
   - 空間計算量: O(1) 追加
   - 採用/非採用理由: 本命
3. 重複 run の読み飛ばしを helper に切り出す
   - 時間計算量: O(n)
   - 空間計算量: O(1) 追加
   - 採用/非採用理由: 整理版として見通しがよい

採用解法の説明:
- `prev` は「答え側の末尾」の直前ノードを指す
- `prev.next` から 1 run 取り出して、その値が重複しているかを調べる
- 重複 run なら全部飛ばして `prev.next` を次の run へつなぐ
- 単独 run ならそのノードを残し、`prev` 自体を前へ進める

Rust観点の解説:
- 先頭が丸ごと消えるケースがあるので、dummy node が有効
- `prev.next.take()` で run の所有権をいったん外に出すと、削除/保持の分岐を安全に書ける
- helper 版では「同じ値の run をどこまで飛ばすか」を関数へ押し込める

いいコード観点の解説:
- 本命実装は「現在の run を読む」「残す/捨てるを決める」という責務が明快
- 83 番との違いをコードで強調するため、`duplicated` の真偽を明示した方が読みやすい
- 先頭削除ケースを特別扱いしないことが、実装の安定性に直結する

落とし穴:
- run の先頭だけ消して、残りの重複値を残してしまう
- 先頭 run が重複するケースで head 更新を忘れる
- 重複がなかった場合に `prev` を進め忘れる
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { next: None, val }
    }
}

struct Solution;
struct NaiveSolution;
struct HelperSolution;

impl NaiveSolution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let values = to_vec(&head);
        let mut filtered = Vec::new();
        let mut i = 0;

        while i < values.len() {
            let mut j = i + 1;
            while j < values.len() && values[j] == values[i] {
                j += 1;
            }
            if j - i == 1 {
                filtered.push(values[i]);
            }
            i = j;
        }

        build_list(&filtered)
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head.take() });
        let mut prev = &mut dummy;

        while let Some(mut node) = prev.next.take() {
            let value = node.val;
            let mut duplicated = false;

            while let Some(next) = node.next.as_ref() {
                if next.val != value {
                    break;
                }
                duplicated = true;
                node.next = node.next.take().and_then(|next| next.next);
            }

            if duplicated {
                prev.next = node.next.take();
            } else {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

impl HelperSolution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head.take() });
        let mut prev = &mut dummy;

        while let Some(node) = prev.next.take() {
            let (unique, rest) = Self::split_run(node);
            prev.next = rest;
            if unique {
                let next = prev.next.take();
                prev.next = Some(Box::new(ListNode {
                    val: Self::tail_value(&next),
                    next,
                }));
                prev = prev.next.as_mut().unwrap();
                prev.next = prev.next.take().and_then(|node| node.next);
            }
        }

        dummy.next
    }

    fn split_run(mut node: Box<ListNode>) -> (bool, Option<Box<ListNode>>) {
        let value = node.val;
        let mut duplicated = false;
        while let Some(next) = node.next.as_ref() {
            if next.val != value {
                break;
            }
            duplicated = true;
            node.next = node.next.take().and_then(|next| next.next);
        }
        if duplicated {
            (false, node.next.take())
        } else {
            (true, Some(node))
        }
    }

    fn tail_value(node: &Option<Box<ListNode>>) -> i32 {
        node.as_ref().map(|node| node.val).unwrap()
    }
}

fn to_vec(node: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut current = node.as_ref();
    while let Some(node) = current {
        values.push(node.val);
        current = node.next.as_ref();
    }
    values
}

fn build_list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &value in values.iter().rev() {
        head = Some(Box::new(ListNode { val: value, next: head }));
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(values: &[i32], expected: &[i32]) {
        let expected = expected.to_vec();
        assert_eq!(to_vec(&NaiveSolution::delete_duplicates(build_list(values))), expected);
        assert_eq!(to_vec(&Solution::delete_duplicates(build_list(values))), expected);
        assert_eq!(to_vec(&HelperSolution::delete_duplicates(build_list(values))), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(&[1, 2, 3, 3, 4, 4, 5], &[1, 2, 5]);
    }

    #[test]
    fn head_run_is_removed() {
        assert_all_solutions(&[1, 1, 1, 2, 3], &[2, 3]);
    }

    #[test]
    fn all_values_removed() {
        assert_all_solutions(&[1, 1, 2, 2], &[]);
    }
}
