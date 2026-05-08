use std::cell::RefCell;
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
    pub fn has_path_sum(root: Node, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(node) => {
                let node = node.borrow();
                let remain = target_sum - node.val;
                if node.left.is_none() && node.right.is_none() {
                    return remain == 0;
                }
                Self::has_path_sum(node.left.clone(), remain) || Self::has_path_sum(node.right.clone(), remain)
            }
        }
    }
}

impl IterativeSolution {
    pub fn has_path_sum(root: Node, target_sum: i32) -> bool {
        let Some(root) = root else { return false; };
        let mut stack = vec![(root, target_sum)];
        while let Some((node, remain)) = stack.pop() {
            let node_ref = node.borrow();
            let next_remain = remain - node_ref.val;
            if node_ref.left.is_none() && node_ref.right.is_none() && next_remain == 0 {
                return true;
            }
            if let Some(left) = node_ref.left.clone() {
                stack.push((left, next_remain));
            }
            if let Some(right) = node_ref.right.clone() {
                stack.push((right, next_remain));
            }
        }
        false
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
    fn path_exists() {
        let root = build_tree(&[
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            Some(1),
        ]);
        assert!(Solution::has_path_sum(root.clone(), 22));
        assert!(IterativeSolution::has_path_sum(root, 22));
    }

    #[test]
    fn path_missing() {
        let root = build_tree(&[Some(1), Some(2), Some(3)]);
        assert!(!Solution::has_path_sum(root.clone(), 5));
        assert!(!IterativeSolution::has_path_sum(root, 5));
    }
}
