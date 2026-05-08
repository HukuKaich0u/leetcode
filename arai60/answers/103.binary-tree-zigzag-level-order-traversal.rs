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
struct DfsSolution;

impl Solution {
    pub fn zigzag_level_order(root: Node) -> Vec<Vec<i32>> {
        let Some(root) = root else { return vec![]; };
        let mut queue = VecDeque::from([root]);
        let mut out = Vec::new();
        let mut left_to_right = true;

        while !queue.is_empty() {
            let size = queue.len();
            let mut level = Vec::with_capacity(size);
            for _ in 0..size {
                let node = queue.pop_front().unwrap();
                let node_ref = node.borrow();
                level.push(node_ref.val);
                if let Some(left) = node_ref.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node_ref.right.clone() {
                    queue.push_back(right);
                }
            }
            if !left_to_right {
                level.reverse();
            }
            out.push(level);
            left_to_right = !left_to_right;
        }
        out
    }
}

impl DfsSolution {
    pub fn zigzag_level_order(root: Node) -> Vec<Vec<i32>> {
        fn dfs(node: Node, depth: usize, out: &mut Vec<Vec<i32>>) {
            let Some(node) = node else { return; };
            if out.len() == depth {
                out.push(Vec::new());
            }
            let node_ref = node.borrow();
            if depth % 2 == 0 {
                out[depth].push(node_ref.val);
            } else {
                out[depth].insert(0, node_ref.val);
            }
            dfs(node_ref.left.clone(), depth + 1, out);
            dfs(node_ref.right.clone(), depth + 1, out);
        }
        let mut out = Vec::new();
        dfs(root, 0, &mut out);
        out
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
        let expected = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(Solution::zigzag_level_order(root.clone()), expected);
        assert_eq!(DfsSolution::zigzag_level_order(root), expected);
    }
}
