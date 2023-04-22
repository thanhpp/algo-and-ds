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

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut p = &mut head;

    loop {
        let mut l1 = match list1 {
            Some(n) => n,
            None => {
                *p = list2;
                break;
            }
        };
        let mut l2 = match list2 {
            Some(n) => n,
            None => {
                *p = Some(l1);
                break;
            }
        };

        if l1.val < l2.val {
            list1 = l1.next.take();
            list2 = Some(l2);
            *p = Some(l1);
        } else {
            list1 = Some(l1);
            list2 = l2.next.take();
            *p = Some(l2);
        }

        p = &mut p.as_mut().unwrap().next;
    }

    head
}
