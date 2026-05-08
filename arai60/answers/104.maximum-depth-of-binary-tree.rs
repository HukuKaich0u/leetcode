use std::cell::RefCell;
use std::cmp::max;
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
struct BfsSolution;
struct StackSolution;

impl Solution {
    pub fn max_depth(root: Node) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                1 + max(Self::max_depth(node.left.clone()), Self::max_depth(node.right.clone()))
            }
        }
    }
}

impl BfsSolution {
    pub fn max_depth(root: Node) -> i32 {
        let Some(root) = root else { return 0; };
        let mut queue = VecDeque::from([(root, 1)]);
        let mut answer = 0;
        while let Some((node, depth)) = queue.pop_front() {
            answer = answer.max(depth);
            let node_ref = node.borrow();
            if let Some(left) = node_ref.left.clone() {
                queue.push_back((left, depth + 1));
            }
            if let Some(right) = node_ref.right.clone() {
                queue.push_back((right, depth + 1));
            }
        }
        answer
    }
}

impl StackSolution {
    pub fn max_depth(root: Node) -> i32 {
        let Some(root) = root else { return 0; };
        let mut stack = vec![(root, 1)];
        let mut answer = 0;
        while let Some((node, depth)) = stack.pop() {
            answer = answer.max(depth);
            let node_ref = node.borrow();
            if let Some(left) = node_ref.left.clone() {
                stack.push((left, depth + 1));
            }
            if let Some(right) = node_ref.right.clone() {
                stack.push((right, depth + 1));
            }
        }
        answer
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
        assert_eq!(Solution::max_depth(root.clone()), 3);
        assert_eq!(BfsSolution::max_depth(root.clone()), 3);
        assert_eq!(StackSolution::max_depth(root), 3);
    }
}
