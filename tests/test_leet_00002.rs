use leetcode::leet_00002::{Solution, ListNode};

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

#[test]
fn test_add_two_numbers_1_1() {
    let l1 = prepare_linked_list(vec![3, 4, 2]);
    let l2 = prepare_linked_list(vec![4, 6, 5]);
    let mut expected = prepare_linked_list(vec![8, 0, 7]);
    let mut result = Solution::add_two_numbers(l1, l2);

    loop {
        if expected == None && result != None { assert!(false); }
        if expected != None && result == None { assert!(false); }
        if expected == None && result == None { break; }

        if let Some(num1) = expected {
            if let Some(num2) = result {
                assert!(num1.val == num2.val, "{} {}", num1.val, num2.val);
                result = num2.next;
            } else {
                assert!(false);
            }
            expected = num1.next;
        } else {
            assert!(false);
        }
    }

    assert!(true);
}

