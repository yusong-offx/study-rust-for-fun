// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut values = Vec::new();
        let mut tmp = head.clone();
        while let Some(node) = tmp {
            values.push(node.val);
            tmp = node.next;
        }
        
        let mut root = None;
        for v in values {
            let mut now = ListNode::new(v);
            now.next = root;
            root = Some(Box::new(now));
        }
        
        root
    }
}