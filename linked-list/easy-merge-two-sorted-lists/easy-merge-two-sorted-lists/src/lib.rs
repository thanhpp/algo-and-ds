mod lib2;

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

// 2ms & 2.1MB
pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // create a pointer to traverse the list 1
    let mut cur1 = &mut list1;

    while list2.is_some() {
        // this solution swap the cur1 between 2 list
        // first, traverse the list 1
        // if the list 1's value is smaller than the one in the list 2 -> continue to traverse list 1
        // else, swap the cur1 to list 2 and continue to traverse through it
        // repeat until the cur1 reaches both 2 lists' end
        if list2.as_ref()?.val < cur1.as_ref()?.val {
            std::mem::swap(cur1, &mut list2)
        }

        cur1 = &mut cur1.as_mut()?.next;

        // reaches the end of 1 list, point to the remaining of the other list then break
        if cur1.is_none() {
            std::mem::swap(cur1, &mut list2);
            break;
        }
    }

    list1
}

/*
    l1: 1       3       5       7      9
          \   /   \   /   \   /   \  /   \
    l2:     2       4       6      8      10
*/
