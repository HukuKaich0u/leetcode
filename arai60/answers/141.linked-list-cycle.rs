/*
この問題で鍛えること:
- アルゴリズム: Floyd's cycle detection
- データ構造: linked list with shared ownership
- Rust での練習ポイント: `Rc<RefCell<_>>`、`Rc::ptr_eq`、共有ノードを持つテスト構築
- コード設計: visited set と O(1) 空間解法の比較

問題の本質:
- cycle があるかどうかだけを知りたい
- 1 歩進む slow と 2 歩進む fast を置くと、cycle 内では差が毎回 1 ずつ縮まる
- cycle がなければ fast が先に `None` へ落ちる

解法候補:
1. visited set
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 直感的だが追加メモリを使う
2. Floyd 法
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命。面接でも必須
3. `advance` helper 付き Floyd
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: ポインタ前進の責務を分離した整理版

採用解法の説明:
- `slow` は 1 歩、`fast` は 2 歩ずつ進める
- cycle があるなら、相対速度 1 で追いかける形になるので、いずれ同じノードへ来る
- cycle がなければ `fast` または `fast.next` が `None` になって終了する

Rust観点の解説:
- LeetCode の cycle 系 linked list は共有ノードを表現するため `Rc<RefCell<_>>` を使う
- 値比較ではなくノード同一性が必要なので `Rc::ptr_eq` を使う
- `advance` を helper 化すると borrow 範囲が短くなり、`RefCell` の扱いが読みやすい

いいコード観点の解説:
- ループ条件を複雑にするより、「進められなければ false」を早期 return する方が明快
- cycle 検出は値ではなくノード identity が本質なので、そこをコード上でも強調したい
- visited set 版を置くと、本命解法がなぜ空間改善になっているかが比較で伝わる

落とし穴:
- 値が同じノード同士を同一と誤認する
- `fast` を 2 歩進める途中で `None` チェックを忘れる
- `Rc<RefCell<_>>` の borrow を長く保持してしまう
*/

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

type Link = Option<Rc<RefCell<ListNode>>>;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Link,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { val, next: None }))
    }
}

struct Solution;
struct NaiveSolution;
struct HelperSolution;

impl NaiveSolution {
    pub fn has_cycle(head: Link) -> bool {
        let mut seen = HashSet::new();
        let mut current = head;

        while let Some(node) = current {
            let ptr = Rc::as_ptr(&node) as usize;
            if !seen.insert(ptr) {
                return true;
            }
            current = node.borrow().next.clone();
        }

        false
    }
}

impl Solution {
    pub fn has_cycle(head: Link) -> bool {
        let mut slow = head.clone();
        let mut fast = head;

        loop {
            slow = match slow {
                Some(node) => node.borrow().next.clone(),
                None => return false,
            };
            fast = match fast {
                Some(node) => match node.borrow().next.clone() {
                    Some(next) => next.borrow().next.clone(),
                    None => return false,
                },
                None => return false,
            };

            match (&slow, &fast) {
                (Some(s), Some(f)) if Rc::ptr_eq(s, f) => return true,
                (None, _) | (_, None) => return false,
                _ => {}
            }
        }
    }
}

impl HelperSolution {
    pub fn has_cycle(head: Link) -> bool {
        let mut slow = head.clone();
        let mut fast = head;

        loop {
            slow = Self::advance(slow, 1);
            fast = Self::advance(fast, 2);

            match (&slow, &fast) {
                (Some(s), Some(f)) if Rc::ptr_eq(s, f) => return true,
                (Some(_), Some(_)) => {}
                _ => return false,
            }
        }
    }

    fn advance(mut node: Link, steps: usize) -> Link {
        for _ in 0..steps {
            node = match node {
                Some(current) => current.borrow().next.clone(),
                None => return None,
            };
        }
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_cycle_list(values: &[i32], cycle_at: Option<usize>) -> Link {
        if values.is_empty() {
            return None;
        }

        let nodes: Vec<_> = values.iter().map(|&value| ListNode::new(value)).collect();
        for pair in nodes.windows(2) {
            pair[0].borrow_mut().next = Some(pair[1].clone());
        }
        if let Some(index) = cycle_at {
            nodes.last().unwrap().borrow_mut().next = Some(nodes[index].clone());
        }
        Some(nodes[0].clone())
    }

    fn assert_all_solutions(values: &[i32], cycle_at: Option<usize>, expected: bool) {
        assert_eq!(NaiveSolution::has_cycle(build_cycle_list(values, cycle_at)), expected);
        assert_eq!(Solution::has_cycle(build_cycle_list(values, cycle_at)), expected);
        assert_eq!(HelperSolution::has_cycle(build_cycle_list(values, cycle_at)), expected);
    }

    #[test]
    fn detects_cycle() {
        assert_all_solutions(&[3, 2, 0, -4], Some(1), true);
    }

    #[test]
    fn detects_no_cycle() {
        assert_all_solutions(&[1, 2], None, false);
    }

    #[test]
    fn single_node_self_cycle() {
        assert_all_solutions(&[1], Some(0), true);
    }
}
