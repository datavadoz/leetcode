/*******************************************************************************
 * [LEET-00002] Add Two Numbers
 * Level: Medium
 * Describtion: You are given two non-empty linked lists representing two
 *      non-negative integers. The digits are stored in reverse order, and each
 *      of their nodes contains a single digit. Add the two numbers and return
 *      the sum as a linked list. You may assume the two numbers do not contain
 *      any leading zero, except the number 0 itself.
 ******************************************************************************/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut extra: i32 = 0;
        let mut tmp: Vec<i32> = Vec::new();

        loop {
            if l1 == None && l2 == None { break; }
            let num1 = match l1 {
                Some(node) => { l1 = node.next; node.val },
                None => 0
            };
            let num2 = match l2 {
                Some(node) => { l2 = node.next; node.val },
                None => 0
            };

            let sum = num1 + num2 + extra;
            let digit = sum % 10;
            extra = sum / 10;
            tmp.push(digit);
        }

        if extra != 0 {
            tmp.push(extra);
        }

        let mut result: Option<Box<ListNode>> = None;
        for i in 0..tmp.len() {
            let index = tmp.len() - i - 1;
            if let Some(node) = result {
                let mut new_node = ListNode::new(tmp[index]);
                new_node.next = Some(node);
                result = Some(Box::new(new_node));
            } else {
                result = Some(Box::new(ListNode::new(tmp[index])));
            }
        }

        return result;
    }
}
