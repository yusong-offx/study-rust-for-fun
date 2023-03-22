
struct Solution;

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut front = Vec::new();
        let mut head = head;
        let mut i = 0;
        while let Some(now) = head {
            front.push(Box::new(ListNode::new(now.val)));
            head = now.next;
            i += 1;
        }

        front.remove((i - n) as usize);

        let mut new_head = None;
        while !front.is_empty() {
            let mut now = front.pop().unwrap();
            now.next = new_head;
            new_head = Some(now);
        }
        new_head
    }
}

fn main() {
    let head = Some(
        Box::new(ListNode{
            val : 1,
            next : Some(Box::new(
                ListNode { val: 2, next: None }
            ))
    }));
    Solution::remove_nth_from_end(head, 1);
}