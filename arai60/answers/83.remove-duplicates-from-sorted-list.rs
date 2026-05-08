/*
この問題で鍛えること:
- アルゴリズム: 隣接重複の除去
- データ構造: sorted linked list
- Rust での練習ポイント: 可変参照を前へ進めるループ
- コード設計: 82 番との違いを小さい実装差として表す力

問題の本質:
- sorted list では重複は隣り合って現れる
- しかもこの問題は「重複値を 1 個だけ残す」ので、同じ値が続く限り `next` を飛ばすだけでよい
- 82 番よりずっと簡単で、削除条件の違いが本質

解法候補:
1. 配列へ落として重複を飛ばしつつ再構築
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 単純だが list 更新の練習としては弱い
2. in-place で `next` を飛ばす
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命
3. 次の異なる値まで進める helper 版
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 単純な抽象化の例として有効

採用解法の説明:
- `current` が指すノードは必ず答えに残す
- `current.next` が同じ値なら、そのノードは削除してよい
- 値が変わるまで `next` を飛ばし、変わったら `current` 自体を 1 つ進める

Rust観点の解説:
- `current = node.next.as_mut()` の形で可変参照を前進させるのが基本
- `node.next.take().and_then(|next| next.next)` で 1 ノード消して次へつなぎ直せる
- helper 版では run 処理のまとまりを切り出しても所有権を崩しにくい

いいコード観点の解説:
- 82 番と違って dummy node は不要で、仕様が弱い分コードも短くなる
- 「今のノードは残す」という不変条件を崩さないことが理解の軸
- 解法差が小さい問題では、抽象化しすぎない本命実装の方が読みやすい

落とし穴:
- 同じ値を 1 個だけ残すべきところで全部消してしまう
- `current` を進めるタイミングを誤ってノードを飛ばしすぎる
- 空 list や 1 要素 list を余計な分岐で汚す
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
        for value in values {
            if filtered.last().copied() != Some(value) {
                filtered.push(value);
            }
        }
        build_list(&filtered)
    }
}

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(node) = current {
            while let Some(next) = node.next.as_ref() {
                if next.val != node.val {
                    break;
                }
                node.next = node.next.take().and_then(|next| next.next);
            }
            current = node.next.as_mut();
        }

        head
    }
}

impl HelperSolution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();
        while let Some(node) = current {
            Self::compress_run(node);
            current = node.next.as_mut();
        }
        head
    }

    fn compress_run(node: &mut Box<ListNode>) {
        while let Some(next) = node.next.as_ref() {
            if next.val != node.val {
                break;
            }
            node.next = node.next.take().and_then(|next| next.next);
        }
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
        assert_all_solutions(&[1, 1, 2], &[1, 2]);
    }

    #[test]
    fn longer_run_case() {
        assert_all_solutions(&[1, 1, 2, 3, 3], &[1, 2, 3]);
    }

    #[test]
    fn already_unique_case() {
        assert_all_solutions(&[1, 2, 3], &[1, 2, 3]);
    }
}
