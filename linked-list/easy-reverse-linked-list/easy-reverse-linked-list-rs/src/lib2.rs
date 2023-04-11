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

// 2ms, 2.6 MB
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr): (Option<Box<ListNode>>, Option<Box<ListNode>>) = (None, head);

    // NONE -> head -> head.Next
    // prev -> curr                 | iter 1
    //         prev -> curr         | iter 2

    while let Some(mut c) = curr {
        // curr.next = prev (reserve the link)
        // curr = curr.next (move the current)
        curr = std::mem::replace(&mut c.next, prev);

        /*
        NOTE:
        - c is a Box, and curr is an Option enum
        - Some(mut c) = curr separates the c from the curr
        - then curr can be update without modifying the c value
        */

        // prev = curr
        prev = Some(c)
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_link_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));

        for i in v.iter().rev() {
            let mut list_node = ListNode::new(*i);
            list_node.next = head.clone().next;
            head.next = Some(Box::new(list_node));
        }

        head.next
    }

    #[test]
    fn test_1() {
        let v = vec![1, 2, 3, 4, 5];

        assert_eq!(
            reverse_list(gen_link_list(v.clone())),
            gen_link_list(v.iter().rev().map(|x| *x).collect::<Vec<i32>>())
        )
    }

    #[test]
    fn test_2() {
        let v = (1..5000).into_iter().collect::<Vec<i32>>();

        assert_eq!(
            reverse_list(gen_link_list(v.clone())),
            gen_link_list(v.iter().rev().map(|x| *x).collect::<Vec<i32>>())
        )
    }
}
