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
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(refcell_now) = node {
            let now = refcell_now.borrow();
            Self::inorder(&now.left, order);
            order.push(now.val);
            Self::inorder(&now.right, order);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer = Vec::new();
        Self::inorder(&root, &mut answer);
        answer
    }
}