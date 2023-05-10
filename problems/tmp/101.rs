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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap().clone();
        let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());
        let (mut stack_a, mut stack_b) = (vec![left], vec![right]);
        while !stack_a.is_empty() && !stack_b.is_empty() {
          let (left, right) = (stack_a.pop().unwrap(), stack_b.pop().unwrap());
          if left.is_none() && right.is_none() {
            continue
          }
          if left.is_some() && right.is_some() {
            let (left, right) = (left.unwrap(), right.unwrap());
            if left.borrow().val == right.borrow().val {
              stack_a.extend(vec![left.borrow().left.clone(), left.borrow().right.clone()]);
              stack_b.extend(vec![right.borrow().right.clone(), right.borrow().left.clone()]);
              continue
            }
          }
          return false
        }
        true
    }
}

