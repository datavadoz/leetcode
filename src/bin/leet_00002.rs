/*******************************************************************************
 * [LEET-00002] Add Two Numbers
 * Level: Medium
 * Describtion: You are given two non-empty linked lists representing two
 *      non-negative integers. The digits are stored in reverse order, and each
 *      of their nodes contains a single digit. Add the two numbers and return
 *      the sum as a linked list. You may assume the two numbers do not contain
 *      any leading zero, except the number 0 itself.
 ******************************************************************************/
use leetcode::leet_00002::{ListNode, Solution};

fn prepare_linked_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut list: Option<Box<ListNode>> = None;

    for element in vector {
        if let Some(node) = list {
            let mut new_node = ListNode::new(element);
            new_node.next = Some(node);
            list = Some(Box::new(new_node));
        } else {
            list = Some(Box::new(ListNode::new(element)));
        }
    }

    return list;
}

fn print_linked_list(list: &Option<Box<ListNode>>) {
    match list {
        Some(node) => {
            print!("{} -> ", node.val);
            print_linked_list(&node.next);
        },
        None => { println!("None"); }
    }
}

fn main() {
    let l1 = prepare_linked_list(vec![3, 4, 2]);
    let l2 = prepare_linked_list(vec![4, 6, 5]);
    print_linked_list(&l1);
    print_linked_list(&l2);
    let result = Solution::add_two_numbers(l1, l2);
    print_linked_list(&result);
}
