use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}

struct Solution;
struct IterativeSolution;

impl Solution {
    pub fn merge_trees(root1: Node, root2: Node) -> Node {
        match (root1, root2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => clone_tree(Some(node)),
            (Some(a), Some(b)) => {
                let a_ref = a.borrow();
                let b_ref = b.borrow();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: a_ref.val + b_ref.val,
                    left: Self::merge_trees(a_ref.left.clone(), b_ref.left.clone()),
                    right: Self::merge_trees(a_ref.right.clone(), b_ref.right.clone()),
                })))
            }
        }
    }
}

impl IterativeSolution {
    pub fn merge_trees(root1: Node, root2: Node) -> Node {
        match (root1, root2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => clone_tree(Some(node)),
            (Some(a), Some(b)) => {
                let root = Rc::new(RefCell::new(TreeNode::new(a.borrow().val + b.borrow().val)));
                let mut queue = VecDeque::from([(root.clone(), a, b)]);
                while let Some((merged, left, right)) = queue.pop_front() {
                    let left_ref = left.borrow();
                    let right_ref = right.borrow();
                    merged.borrow_mut().left = merge_child(left_ref.left.clone(), right_ref.left.clone(), &mut queue);
                    merged.borrow_mut().right = merge_child(left_ref.right.clone(), right_ref.right.clone(), &mut queue);
                }
                Some(root)
            }
        }
    }
}

fn merge_child(a: Node, b: Node, queue: &mut VecDeque<(Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>, Rc<RefCell<TreeNode>>)>) -> Node {
    match (a, b) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => clone_tree(Some(node)),
        (Some(left), Some(right)) => {
            let merged = Rc::new(RefCell::new(TreeNode::new(left.borrow().val + right.borrow().val)));
            queue.push_back((merged.clone(), left, right));
            Some(merged)
        }
    }
}

fn clone_tree(root: Node) -> Node {
    let node = root?;
    let node_ref = node.borrow();
    Some(Rc::new(RefCell::new(TreeNode {
        val: node_ref.val,
        left: clone_tree(node_ref.left.clone()),
        right: clone_tree(node_ref.right.clone()),
    })))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(values: &[Option<i32>]) -> Node {
        if values.is_empty() || values[0].is_none() {
            return None;
        }
        let nodes: Vec<_> = values
            .iter()
            .map(|value| value.map(|value| Rc::new(RefCell::new(TreeNode::new(value)))))
            .collect();
        for i in 0..values.len() {
            if let Some(node) = &nodes[i] {
                node.borrow_mut().left = nodes.get(2 * i + 1).cloned().unwrap_or(None);
                node.borrow_mut().right = nodes.get(2 * i + 2).cloned().unwrap_or(None);
            }
        }
        nodes[0].clone()
    }

    fn level_order(root: Node) -> Vec<Option<i32>> {
        let Some(root) = root else { return vec![]; };
        let mut out = Vec::new();
        let mut queue = VecDeque::from([Some(root)]);
        while let Some(node) = queue.pop_front() {
            match node {
                None => out.push(None),
                Some(node) => {
                    let node_ref = node.borrow();
                    out.push(Some(node_ref.val));
                    queue.push_back(node_ref.left.clone());
                    queue.push_back(node_ref.right.clone());
                }
            }
        }
        while out.last() == Some(&None) {
            out.pop();
        }
        out
    }

    #[test]
    fn basic_case() {
        let root1 = build_tree(&[Some(1), Some(3), Some(2), Some(5)]);
        let root2 = build_tree(&[Some(2), Some(1), Some(3), None, Some(4), None, Some(7)]);
        let expected = vec![Some(3), Some(4), Some(5), Some(5), Some(4), None, Some(7)];
        assert_eq!(level_order(Solution::merge_trees(root1.clone(), root2.clone())), expected);
        assert_eq!(level_order(IterativeSolution::merge_trees(root1, root2)), expected);
    }
}
