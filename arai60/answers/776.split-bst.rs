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
struct CloneSolution;

impl Solution {
    pub fn split_bst(root: Node, target: i32) -> Vec<Node> {
        fn dfs(root: Node, target: i32) -> (Node, Node) {
            let Some(node) = root else { return (None, None); };
            if node.borrow().val <= target {
                let right = node.borrow_mut().right.take();
                let (small, large) = dfs(right, target);
                node.borrow_mut().right = small;
                (Some(node), large)
            } else {
                let left = node.borrow_mut().left.take();
                let (small, large) = dfs(left, target);
                node.borrow_mut().left = large;
                (small, Some(node))
            }
        }

        let (left, right) = dfs(root, target);
        vec![left, right]
    }
}

impl CloneSolution {
    pub fn split_bst(root: Node, target: i32) -> Vec<Node> {
        let left = build_filtered(root.clone(), target, true);
        let right = build_filtered(root, target, false);
        vec![left, right]
    }
}

fn build_filtered(root: Node, target: i32, take_small: bool) -> Node {
    let node = root?;
    let node_ref = node.borrow();
    let keep = if take_small {
        node_ref.val <= target
    } else {
        node_ref.val > target
    };
    if keep {
        Some(Rc::new(RefCell::new(TreeNode {
            val: node_ref.val,
            left: build_filtered(node_ref.left.clone(), target, take_small),
            right: build_filtered(node_ref.right.clone(), target, take_small),
        })))
    } else if take_small {
        build_filtered(node_ref.left.clone(), target, take_small)
    } else {
        build_filtered(node_ref.right.clone(), target, take_small)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_bst(values: &[i32]) -> Node {
        fn insert(root: &mut Node, value: i32) {
            match root {
                None => *root = Some(Rc::new(RefCell::new(TreeNode::new(value)))),
                Some(node) => {
                    if value < node.borrow().val {
                        insert(&mut node.borrow_mut().left, value);
                    } else {
                        insert(&mut node.borrow_mut().right, value);
                    }
                }
            }
        }

        let mut root = None;
        for &value in values {
            insert(&mut root, value);
        }
        root
    }

    fn inorder(node: Node, out: &mut Vec<i32>) {
        let Some(node) = node else { return; };
        let node_ref = node.borrow();
        inorder(node_ref.left.clone(), out);
        out.push(node_ref.val);
        inorder(node_ref.right.clone(), out);
    }

    #[test]
    fn basic_case() {
        let root = build_bst(&[4, 2, 6, 1, 3, 5, 7]);
        let parts = Solution::split_bst(root, 2);
        let mut left = Vec::new();
        let mut right = Vec::new();
        inorder(parts[0].clone(), &mut left);
        inorder(parts[1].clone(), &mut right);
        assert_eq!(left, vec![1, 2]);
        assert_eq!(right, vec![3, 4, 5, 6, 7]);

        let root = build_bst(&[4, 2, 6, 1, 3, 5, 7]);
        let parts = CloneSolution::split_bst(root, 2);
        left.clear();
        right.clear();
        inorder(parts[0].clone(), &mut left);
        inorder(parts[1].clone(), &mut right);
        assert_eq!(left, vec![1, 2]);
        assert_eq!(right, vec![3, 4, 5, 6, 7]);
    }
}
