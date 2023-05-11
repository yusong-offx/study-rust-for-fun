use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    fn recur(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> (i32, bool) {
        if root.is_none() {
            return (0, false)
        }

        let mut cnt = 1;
        let now = root.unwrap();

        let (c, h) = Self::recur(now.borrow().left.clone(), k);
        if h {
            return (c, h);
        }
        cnt += c;
        if k == cnt {
            return (now.borrow().val, true)
        }
        

        let (c, h) = Self::recur(now.borrow().right.clone(), k-cnt);
        cnt += c;
        if h {
            return (c, h);
        }
        if k == cnt {
            return (now.borrow().val, true)
        }

        (cnt, false)
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::recur(root, k).0
    }
}