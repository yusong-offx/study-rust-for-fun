// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0
        }
        let mut max_depth = 0;
        let mut stack = vec![(root.unwrap(), 1)];
        while !stack.is_empty() {
            let (now, depth) = stack.pop().unwrap();
            max_depth = max_depth.max(depth);
            if now.borrow().left.is_some() {
                let left = now.borrow().left.clone().unwrap();
                stack.push((left, depth+1));
            }
            if now.borrow().right.is_some() {
                let right = now.borrow().right.clone().unwrap();
                stack.push((right, depth+1));
            }

        }

        max_depth
    }
}