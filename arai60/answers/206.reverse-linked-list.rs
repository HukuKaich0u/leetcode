/*
この問題で鍛えること:
- アルゴリズム: ポインタ更新の逐次シミュレーション
- データ構造: singly linked list
- Rust での練習ポイント: `take()` による辺の切り離し、`Option<Box<_>>` の再接続
- コード設計: 3 ポインタ反転と再帰版の比較

問題の本質:
- 今見ているノードの向きを反転するには、次ノードを失う前に退避する必要がある
- つまり本質は「どの順で辺を切り替えるか」であって、複雑なデータ処理ではない

解法候補:
1. 配列へ落として逆順で作り直す
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 発想は簡単だが linked list の練習にならない
2. 反復 3 ポインタ
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命。最重要パターン
3. 再帰
   - 時間計算量: O(n)
   - 空間計算量: O(n) stack
   - 採用/非採用理由: きれいだが反復の方が制御しやすい

採用解法の説明:
- `current` がまだ未処理の先頭、`prev` がすでに反転済みの先頭を表す
- 1 ステップでやることは 3 つだけ
  - `next` を退避
  - `current.next = prev` に張り替え
  - `prev = current`, `current = next` で前進
- ループが終わった時点で `prev` が新しい head

Rust観点の解説:
- `node.next.take()` で next を安全に切り離すと、その後の所有権移動が明確になる
- `while let Some(mut node) = current` の形にすると、現在ノードの所有権をその場で受け取れる
- 再帰版は helper の返り値を `Option<Box<ListNode>>` にすると LeetCode 形式と揃う

いいコード観点の解説:
- 反転は「1 ステップで不変条件をどう更新するか」が読みやすさの核心
- 本命実装は変数名を短くしても意味が崩れにくいが、`prev/current/next` 以外にすると理解しづらい
- 再帰版は理解の補助にはなるが、まず反復版を自力で説明できることが重要

落とし穴:
- `next` を退避する前に `node.next` を上書きして残りの list を失う
- `prev` と `current` の更新順を崩す
- 空 list や 1 要素 list を特別扱いしすぎてコードを複雑にする
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
struct RecursiveSolution;

impl NaiveSolution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut values = Vec::new();
        let mut current = head.as_ref();
        while let Some(node) = current {
            values.push(node.val);
            current = node.next.as_ref();
        }
        values.reverse();
        build_list(&values)
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut prev = None;

        // `prev` は反転済み部分の先頭、`current` は未処理部分の先頭を表す。
        while let Some(mut node) = current {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }

        prev
    }
}

impl RecursiveSolution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::reverse(head, None)
    }

    fn reverse(current: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let Some(mut node) = current else {
            return prev;
        };
        let next = node.next.take();
        node.next = prev;
        Self::reverse(next, Some(node))
    }
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

    fn to_vec(node: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut values = Vec::new();
        let mut current = node.as_ref();
        while let Some(node) = current {
            values.push(node.val);
            current = node.next.as_ref();
        }
        values
    }

    fn assert_all_solutions(values: &[i32], expected: &[i32]) {
        let expected = expected.to_vec();
        assert_eq!(to_vec(&NaiveSolution::reverse_list(build_list(values))), expected);
        assert_eq!(to_vec(&Solution::reverse_list(build_list(values))), expected);
        assert_eq!(to_vec(&RecursiveSolution::reverse_list(build_list(values))), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(&[1, 2, 3, 4, 5], &[5, 4, 3, 2, 1]);
    }

    #[test]
    fn single_element_case() {
        assert_all_solutions(&[1], &[1]);
    }

    #[test]
    fn empty_case() {
        assert_all_solutions(&[], &[]);
    }
}
