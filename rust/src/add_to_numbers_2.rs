// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug, Default)]
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
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_two_numbers_helper(l1, l2, 0)
}

fn add_two_numbers_helper(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        return None;
    }

    let l1 = l1.unwrap_or(Box::new(ListNode::new(0)));
    let l2 = l2.unwrap_or(Box::new(ListNode::new(0)));

    let sum = l1.val + l2.val + carry;
    let mut current_node = ListNode::new(sum % 10);
    current_node.next = add_two_numbers_helper(l1.next, l2.next, sum / 10);

    Some(Box::new(current_node))
}
