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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![(root.unwrap(), i64::MIN, i64::MAX)];
        while !stack.is_empty() {
            let (now, low, high) = stack.pop().unwrap();

            let now = now.borrow();
            let val = now.val as i64;
            if val <= low || val >= high{
                return false
            }
            if let Some(left) = now.left.clone() {
                stack.push((left, low, val));
                
            }
            if let Some(right) = now.right.clone(){
                stack.push((right, val, high));
            }
            
        }

        true
    }
}