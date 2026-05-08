/*
この問題で鍛えること:
- アルゴリズム: Floyd 法の第 2 段階
- データ構造: shared linked list
- Rust での練習ポイント: `Rc::ptr_eq` によるノード一致判定、meeting point の切り出し
- コード設計: まず cycle を見つけ、その後入口を求める 2 段構成

問題の本質:
- 141 番より一段深く、cycle があるかだけでなく入口ノードを返す必要がある
- 会合点が見つかった後、head からの距離と会合点から入口までの距離が一致する
- だから `head` と会合点から同速で進めると入口でぶつかる

解法候補:
1. visited set で最初の再訪ノードを返す
   - 時間計算量: O(n)
   - 空間計算量: O(n)
   - 採用/非採用理由: わかりやすいが追加空間が必要
2. Floyd 法 2 段階
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 本命
3. 会合点探索を helper に切り出す
   - 時間計算量: O(n)
   - 空間計算量: O(1)
   - 採用/非採用理由: 数式と実装の対応が見えやすい整理版

採用解法の説明:
- まず 141 番と同じく slow/fast で会合点を探す
- 会合点がなければ cycle は存在しない
- 会合点が得られたら、`p1 = head`, `p2 = meet` として同速で進める
- 最初に一致したノードが cycle の入口

Rust観点の解説:
- 入口を返すので、`Link` を clone しながら pointer identity を維持する
- 会合点探索 helper は `Option<Rc<RefCell<_>>>` を返すと main ロジックが読みやすい
- borrow をまたいで `Rc::ptr_eq` しないよう、clone した `Rc` 同士で比較する

いいコード観点の解説:
- 「cycle 判定」と「入口探索」を段階分けすると理屈が崩れにくい
- 会合点探索を helper 化すると、入口探索の 2 ポインタロジックだけを主役にできる
- visited set 版を残すと、答えの意味と追加空間のトレードオフが明快

落とし穴:
- 会合点が入口そのものだと決めつける
- ノードの値が同じなら同じノードだと誤解する
- cycle がないケースで第 2 段階へ進んで panic する
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
    pub fn detect_cycle(head: Link) -> Link {
        let mut seen = HashSet::new();
        let mut current = head;

        while let Some(node) = current {
            let ptr = Rc::as_ptr(&node) as usize;
            if !seen.insert(ptr) {
                return Some(node);
            }
            current = node.borrow().next.clone();
        }

        None
    }
}

impl Solution {
    pub fn detect_cycle(head: Link) -> Link {
        let meet = HelperSolution::find_meeting_point(head.clone())?;
        let mut p1 = head;
        let mut p2 = Some(meet);

        while let (Some(a), Some(b)) = (p1.clone(), p2.clone()) {
            if Rc::ptr_eq(&a, &b) {
                return Some(a);
            }
            p1 = a.borrow().next.clone();
            p2 = b.borrow().next.clone();
        }

        None
    }
}

impl HelperSolution {
    pub fn detect_cycle(head: Link) -> Link {
        let meet = Self::find_meeting_point(head.clone())?;
        let mut head_ptr = head;
        let mut meet_ptr = Some(meet);

        loop {
            match (head_ptr.clone(), meet_ptr.clone()) {
                (Some(a), Some(b)) if Rc::ptr_eq(&a, &b) => return Some(a),
                (Some(a), Some(b)) => {
                    head_ptr = a.borrow().next.clone();
                    meet_ptr = b.borrow().next.clone();
                }
                _ => return None,
            }
        }
    }

    fn find_meeting_point(head: Link) -> Link {
        let mut slow = head.clone();
        let mut fast = head;

        loop {
            slow = match slow {
                Some(node) => node.borrow().next.clone(),
                None => return None,
            };
            fast = match fast {
                Some(node) => match node.borrow().next.clone() {
                    Some(next) => next.borrow().next.clone(),
                    None => return None,
                },
                None => return None,
            };

            match (&slow, &fast) {
                (Some(s), Some(f)) if Rc::ptr_eq(s, f) => return slow,
                (None, _) | (_, None) => return None,
                _ => {}
            }
        }
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

    fn node_value(node: Link) -> Option<i32> {
        node.map(|node| node.borrow().val)
    }

    fn assert_all_solutions(values: &[i32], cycle_at: Option<usize>, expected: Option<i32>) {
        assert_eq!(node_value(NaiveSolution::detect_cycle(build_cycle_list(values, cycle_at))), expected);
        assert_eq!(node_value(Solution::detect_cycle(build_cycle_list(values, cycle_at))), expected);
        assert_eq!(node_value(HelperSolution::detect_cycle(build_cycle_list(values, cycle_at))), expected);
    }

    #[test]
    fn detects_cycle_entry() {
        assert_all_solutions(&[3, 2, 0, -4], Some(1), Some(2));
    }

    #[test]
    fn returns_none_without_cycle() {
        assert_all_solutions(&[1, 2], None, None);
    }

    #[test]
    fn self_cycle_entry() {
        assert_all_solutions(&[1], Some(0), Some(1));
    }
}
