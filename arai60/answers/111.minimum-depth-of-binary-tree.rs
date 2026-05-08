use std::cell::RefCell;
use std::cmp::min;
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
struct DfsSolution;

impl Solution {
    pub fn min_depth(root: Node) -> i32 {
        let Some(root) = root else { return 0; };
        let mut queue = VecDeque::from([(root, 1)]);
        while let Some((node, depth)) = queue.pop_front() {
            let node_ref = node.borrow();
            if node_ref.left.is_none() && node_ref.right.is_none() {
                return depth;
            }
            if let Some(left) = node_ref.left.clone() {
                queue.push_back((left, depth + 1));
            }
            if let Some(right) = node_ref.right.clone() {
                queue.push_back((right, depth + 1));
            }
        }
        0
    }
}

impl DfsSolution {
    pub fn min_depth(root: Node) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                match (node.left.clone(), node.right.clone()) {
                    (None, None) => 1,
                    (Some(left), None) => 1 + Self::min_depth(Some(left)),
                    (None, Some(right)) => 1 + Self::min_depth(Some(right)),
                    (Some(left), Some(right)) => 1 + min(Self::min_depth(Some(left)), Self::min_depth(Some(right))),
                }
            }
        }
    }
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

    #[test]
    fn basic_case() {
        let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::min_depth(root.clone()), 2);
        assert_eq!(DfsSolution::min_depth(root), 2);
    }
}
