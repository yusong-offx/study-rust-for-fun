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
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        let mut dq = VecDeque::new();
        if let Some(start) = root {
            dq.push_back(start);
        } else {
            return answer
        }
        let mut rev = false;
        while !dq.is_empty() {
            let mut tmp = VecDeque::new();
            let mut frag = Vec::new();

            while !dq.is_empty() {
                let nxt = dq.pop_front().unwrap();
                let now = nxt.borrow_mut();
                frag.push(now.val);
                if let Some(left) = now.left.clone() {
                    tmp.push_back(left);
                }
                if let Some(right) = now.right.clone() {
                    tmp.push_back(right);
                }
            }
            if rev {
                frag.reverse();
            }
            rev = !rev;
            answer.push(frag);
            dq = tmp;
        }
    
        answer
    }
}
