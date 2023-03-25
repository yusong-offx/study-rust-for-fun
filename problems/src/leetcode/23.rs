use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for list in lists {
            let mut el = list;
            while let Some(now) = el {
                heap.push(now.val);
                el = now.next;
            }
        }
        let mut head = None;
        while !heap.is_empty() {
            let val = heap.pop().unwrap();
            head = Some(Box::new(ListNode{val, next:head}));
        }
        head
    }
}