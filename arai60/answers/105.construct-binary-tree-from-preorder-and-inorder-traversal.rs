use std::cell::RefCell;
use std::collections::HashMap;
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
struct SliceSolution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
        let index: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();

        fn dfs(
            preorder: &[i32],
            pre_left: usize,
            pre_right: usize,
            in_left: usize,
            in_right: usize,
            index: &HashMap<i32, usize>,
        ) -> Node {
            if pre_left >= pre_right || in_left >= in_right {
                return None;
            }
            let root_val = preorder[pre_left];
            let mid = index[&root_val];
            let left_size = mid - in_left;
            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left: dfs(preorder, pre_left + 1, pre_left + 1 + left_size, in_left, mid, index),
                right: dfs(preorder, pre_left + 1 + left_size, pre_right, mid + 1, in_right, index),
            })))
        }

        dfs(&preorder, 0, preorder.len(), 0, inorder.len(), &index)
    }
}

impl SliceSolution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Node {
        fn dfs(preorder: &[i32], inorder: &[i32]) -> Node {
            if preorder.is_empty() {
                return None;
            }
            let root_val = preorder[0];
            let mid = inorder.iter().position(|&value| value == root_val).unwrap();
            Some(Rc::new(RefCell::new(TreeNode {
                val: root_val,
                left: dfs(&preorder[1..1 + mid], &inorder[..mid]),
                right: dfs(&preorder[1 + mid..], &inorder[mid + 1..]),
            })))
        }
        dfs(&preorder, &inorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn preorder(node: Node, out: &mut Vec<i32>) {
        let Some(node) = node else { return; };
        let node_ref = node.borrow();
        out.push(node_ref.val);
        preorder(node_ref.left.clone(), out);
        preorder(node_ref.right.clone(), out);
    }

    fn inorder(node: Node, out: &mut Vec<i32>) {
        let Some(node) = node else { return; };
        let node_ref = node.borrow();
        inorder(node_ref.left.clone(), out);
        out.push(node_ref.val);
        inorder(node_ref.right.clone(), out);
    }

    fn assert_solution(root: Node, preorder_expected: &[i32], inorder_expected: &[i32]) {
        let mut pre = Vec::new();
        let mut ino = Vec::new();
        preorder(root.clone(), &mut pre);
        inorder(root, &mut ino);
        assert_eq!(pre, preorder_expected);
        assert_eq!(ino, inorder_expected);
    }

    #[test]
    fn basic_case() {
        let preorder_values = vec![3, 9, 20, 15, 7];
        let inorder_values = vec![9, 3, 15, 20, 7];
        assert_solution(
            Solution::build_tree(preorder_values.clone(), inorder_values.clone()),
            &preorder_values,
            &inorder_values,
        );
        assert_solution(
            SliceSolution::build_tree(preorder_values.clone(), inorder_values.clone()),
            &preorder_values,
            &inorder_values,
        );
    }
}
