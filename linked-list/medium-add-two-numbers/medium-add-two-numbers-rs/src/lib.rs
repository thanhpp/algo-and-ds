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

// 4ms, 2.2 MB
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let dummy = ListNode::new(0);
    let mut dummy = Box::new(dummy);
    let mut curr = dummy.as_mut();
    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let v1 = match l1 {
            None => 0,
            Some(n) => {
                l1 = n.next;
                n.val
            }
        };

        let v2 = match l2 {
            None => 0,
            Some(n) => {
                l2 = n.next;
                n.val
            }
        };

        let mut val = v1 + v2 + carry;
        carry = val / 10;
        val = val % 10;

        curr.next = Some(Box::new(ListNode::new(val)));
        curr = curr.next.as_mut().unwrap();
    }

    dummy.next
}
