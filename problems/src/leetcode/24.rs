// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut stack = Vec::new();
        while head.is_some() {
            let now = head.unwrap();
            stack.push(now.val);
            head = now.next;
        }
        for i in (0..stack.len()).step_by(2) {
            if i + 1 < stack.len() {
                let tmp = stack[i];
                stack[i] = stack[i + 1];
                stack[i + 1] = tmp;
            }
        }
        let mut prev = None;
        while !stack.is_empty() {
            let now = stack.pop().unwrap();
            let mut node = ListNode::new(now);
            node.next = prev;
            prev = Some(Box::new(node));
        }
        prev
    }
}

fn main() {
}