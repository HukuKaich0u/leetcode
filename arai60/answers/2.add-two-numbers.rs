/*
この問題で鍛えること:
- アルゴリズム: 桁ごとのシミュレーション
- データ構造: singly linked list
- Rust での練習ポイント: `Option<Box<_>>` の前進、dummy node、所有権を壊さない tail 更新
- コード設計: 筆算の本質だけを残したループと、補助関数へ切り出した実装の差

問題の本質:
- 各ノードは 1 桁だけ持ち、桁は下位から並んでいる
- つまり配列へ並べ替えなくても、前から順に「今の 2 桁 + carry」だけ見れば答えを確定できる
- 全体の状態は大きく見えても、本当に必要なのは局所状態だけという典型例

解法候補:
1. 配列へ落としてから足す
   - 時間計算量: O(n + m)
   - 空間計算量: O(n + m)
   - 採用/非採用理由: 発想は素直だが、linked list をその場で扱う練習にはならない
2. 反復で筆算する
   - 時間計算量: O(n + m)
   - 空間計算量: O(1) 追加
   - 採用/非採用理由: 本命。面接でも実務でも説明しやすい
3. 取得処理を helper に切り出す
   - 時間計算量: O(n + m)
   - 空間計算量: O(1) 追加
   - 採用/非採用理由: 責務分離を見せやすい整理版

採用解法の説明:
- ループ 1 回で 1 桁ぶんの答えを確定する
- `v1 + v2 + carry` を計算し、`sum % 10` を現在桁、`sum / 10` を次の carry にする
- どちらかの list が尽きても `0` とみなせば同じループを続けられる
- 末尾で carry だけ残るケースも同じ条件に含めると、分岐が減って壊れにくい

Rust観点の解説:
- `dummy` を使うと先頭ノードだけ特別扱いせずに tail append できる
- `and_then(|node| node.next)` で所有権を前へ進めると、古いノードを安全に消費できる
- helper 版では「次の値を 1 個読む」責務を関数化して、メインループを筆算の骨格だけにできる

いいコード観点の解説:
- 本命実装では `while l1.is_some() || l2.is_some() || carry > 0` が終了条件を一箇所へ集約している
- 整理版は読みやすさが上がる一方、抽象化しすぎると linked list 更新の見通しを失いやすい
- 素朴実装を併記すると、同じ正しさでもデータ構造を活かしているかどうかを比較できる

落とし穴:
- 片方の list が短いときに `None` を `0` とみなす処理を忘れる
- 最後の carry を捨てる
- tail を進め忘れて最後のノードだけ上書きする
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let digits1 = Self::to_digits(&l1);
        let digits2 = Self::to_digits(&l2);
        let mut answer = Vec::new();
        let mut carry = 0;
        let mut i = 0;

        while i < digits1.len() || i < digits2.len() || carry > 0 {
            let v1 = digits1.get(i).copied().unwrap_or(0);
            let v2 = digits2.get(i).copied().unwrap_or(0);
            let sum = v1 + v2 + carry;
            answer.push(sum % 10);
            carry = sum / 10;
            i += 1;
        }

        build_list(&answer)
    }

    fn to_digits(node: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut digits = Vec::new();
        let mut current = node.as_ref();
        while let Some(node) = current {
            digits.push(node.val);
            current = node.next.as_ref();
        }
        digits
    }
}

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        // 片方が尽きても 0 とみなし、carry まで含めて同じループで処理する。
        while l1.is_some() || l2.is_some() || carry > 0 {
            let v1 = l1.as_ref().map_or(0, |node| node.val);
            let v2 = l2.as_ref().map_or(0, |node| node.val);
            let sum = v1 + v2 + carry;
            carry = sum / 10;

            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        dummy.next
    }
}

impl HelperSolution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let sum = Self::take_digit(&mut l1) + Self::take_digit(&mut l2) + carry;
            carry = sum / 10;
            tail = Self::append(tail, sum % 10);
        }

        dummy.next
    }

    fn take_digit(list: &mut Option<Box<ListNode>>) -> i32 {
        if let Some(mut node) = list.take() {
            let value = node.val;
            *list = node.next.take();
            value
        } else {
            0
        }
    }

    fn append(tail: &mut Box<ListNode>, digit: i32) -> &mut Box<ListNode> {
        tail.next = Some(Box::new(ListNode::new(digit)));
        tail.next.as_mut().unwrap()
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

    fn assert_all_solutions(lhs: &[i32], rhs: &[i32], expected: &[i32]) {
        let expected = expected.to_vec();
        let actual = Solution::add_two_numbers(build_list(lhs), build_list(rhs));
        assert_eq!(to_vec(&actual), expected);

        let actual = NaiveSolution::add_two_numbers(build_list(lhs), build_list(rhs));
        assert_eq!(to_vec(&actual), expected);

        let actual = HelperSolution::add_two_numbers(build_list(lhs), build_list(rhs));
        assert_eq!(to_vec(&actual), expected);
    }

    #[test]
    fn basic_case() {
        assert_all_solutions(&[2, 4, 3], &[5, 6, 4], &[7, 0, 8]);
    }

    #[test]
    fn carry_propagates_to_new_tail() {
        assert_all_solutions(&[9, 9, 9, 9, 9, 9, 9], &[9, 9, 9, 9], &[8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn zero_lists() {
        assert_all_solutions(&[0], &[0], &[0]);
    }
}
