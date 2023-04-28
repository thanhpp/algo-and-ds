// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 0ms, 2.1MB
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // using the fast, slow pointer method.
    // the fast pointer will move through n nodes before the slow pointer start moving
    // -> the slow pointer is always after the fast pointer n nodes

    // use a dummy head
    let mut dummy = ListNode::new(0);
    dummy.next = head;
    let mut dummy = Box::new(dummy);

    let (mut fast, mut slow) = (dummy.clone(), dummy.as_mut());

    // move the fast pointer forward n step
    for _ in 0..n {
        if let Some(f) = fast.next {
            fast = f;
        }
    }

    // move forward 2 pointers
    while let Some(f) = fast.next {
        fast = f;
        slow = match slow.next.as_mut() {
            None => return None,
            Some(s) => s,
        };
    }

    let remove = match slow.next.as_mut() {
        Some(s) => s,
        None => return None,
    };
    slow.next = remove.next.take();

    dummy.next
}
