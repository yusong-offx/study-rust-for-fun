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

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn make_ele(fragment: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match fragment.len() {
            0 => None,
            1 => {
                let node = TreeNode::new(fragment[0]);
                Some(Rc::new(RefCell::new(node)))
            },
            _ => {
                let mid = fragment.len() / 2;
                let mut node = TreeNode::new(fragment[mid]);
                node.val = fragment[mid];
                node.left = Self::make_ele(fragment[0..mid].to_vec());
                node.right = Self::make_ele(fragment[mid+1..fragment.len()].to_vec());
                Some(Rc::new(RefCell::new(node)))
            }, 
        }
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::make_ele(nums)
    }
}