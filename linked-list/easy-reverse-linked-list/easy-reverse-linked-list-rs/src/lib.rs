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

// 384ms, 2.8MB
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut h = Box::new(ListNode::new(0));
    let head = head.unwrap();
    h.next = Some(Box::new(ListNode::new(head.val)));
    let mut p = head.next;

    while let Some(n) = p.clone() {
        let mut new_list_node = Box::new(ListNode::new(n.val));
        new_list_node.next = h.next;
        h.next = Some(new_list_node);
        p = n.next;
    }

    h.next
}

#[cfg(test)]
mod tests {
    use crate::ListNode;

    fn gen_link_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));

        for i in v.iter().rev() {
            let mut list_node = ListNode::new(*i);
            list_node.next = head.clone().next;
            head.next = Some(Box::new(list_node));
        }

        head.next
    }

    use super::*;

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
