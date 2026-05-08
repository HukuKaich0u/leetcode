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
struct IndexRangeSolution;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
        fn dfs(nums: &[i32]) -> Node {
            if nums.is_empty() {
                return None;
            }
            let mid = nums.len() / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: dfs(&nums[..mid]),
                right: dfs(&nums[mid + 1..]),
            })))
        }
        dfs(&nums)
    }
}

impl IndexRangeSolution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Node {
        fn dfs(nums: &[i32], left: usize, right: usize) -> Node {
            if left >= right {
                return None;
            }
            let mid = left + (right - left) / 2;
            Some(Rc::new(RefCell::new(TreeNode {
                val: nums[mid],
                left: dfs(nums, left, mid),
                right: dfs(nums, mid + 1, right),
            })))
        }
        dfs(&nums, 0, nums.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inorder(root: Node, out: &mut Vec<i32>) {
        let Some(node) = root else { return; };
        let node_ref = node.borrow();
        inorder(node_ref.left.clone(), out);
        out.push(node_ref.val);
        inorder(node_ref.right.clone(), out);
    }

    fn is_balanced(root: Node) -> bool {
        fn height(node: Node) -> Option<i32> {
            let Some(node) = node else { return Some(0); };
            let node_ref = node.borrow();
            let left = height(node_ref.left.clone())?;
            let right = height(node_ref.right.clone())?;
            if (left - right).abs() > 1 {
                return None;
            }
            Some(1 + left.max(right))
        }
        height(root).is_some()
    }

    fn assert_solution(root: Node, nums: &[i32]) {
        let mut actual = Vec::new();
        inorder(root.clone(), &mut actual);
        assert_eq!(actual, nums);
        assert!(is_balanced(root));
    }

    #[test]
    fn basic_case() {
        let nums = vec![-10, -3, 0, 5, 9];
        assert_solution(Solution::sorted_array_to_bst(nums.clone()), &nums);
        assert_solution(IndexRangeSolution::sorted_array_to_bst(nums.clone()), &nums);
    }
}
