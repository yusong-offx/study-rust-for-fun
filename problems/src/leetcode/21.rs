use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        let mut it1 = list1;
        while let Some(l) = it1 {
            heap.push(l.val);
            it1 = l.next;
        }

        let mut it2 = list2;
        while let Some(l) = it2 {
            heap.push(l.val);
            it2 = l.next;
        }

        let v = heap.into_sorted_vec();

        let mut ans = None;

        for ele in v.iter().rev() {
            ans = Some(Box::new(ListNode{
                val: *ele,
                next: ans,
            }))
        }

        ans
    }
}