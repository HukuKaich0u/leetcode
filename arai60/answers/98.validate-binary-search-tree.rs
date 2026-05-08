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
struct InorderSolution;

impl Solution {
    pub fn is_valid_bst(root: Node) -> bool {
        fn dfs(node: Node, low: Option<i64>, high: Option<i64>) -> bool {
            let Some(node) = node else { return true; };
            let node_ref = node.borrow();
            let value = node_ref.val as i64;
            if low.is_some_and(|limit| value <= limit) || high.is_some_and(|limit| value >= limit) {
                return false;
            }
            dfs(node_ref.left.clone(), low, Some(value)) && dfs(node_ref.right.clone(), Some(value), high)
        }
        dfs(root, None, None)
    }
}

impl InorderSolution {
    pub fn is_valid_bst(root: Node) -> bool {
        fn inorder(node: Node, prev: &mut Option<i64>) -> bool {
            let Some(node) = node else { return true; };
            let node_ref = node.borrow();
            if !inorder(node_ref.left.clone(), prev) {
                return false;
            }
            let value = node_ref.val as i64;
            if prev.is_some_and(|p| value <= p) {
                return false;
            }
            *prev = Some(value);
            inorder(node_ref.right.clone(), prev)
        }
        inorder(root, &mut None)
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
    fn valid_case() {
        let root = build_tree(&[Some(2), Some(1), Some(3)]);
        assert!(Solution::is_valid_bst(root.clone()));
        assert!(InorderSolution::is_valid_bst(root));
    }

    #[test]
    fn invalid_case() {
        let root = build_tree(&[Some(5), Some(1), Some(4), None, None, Some(3), Some(6)]);
        assert!(!Solution::is_valid_bst(root.clone()));
        assert!(!InorderSolution::is_valid_bst(root));
    }
}
