use crate::util::linked_list::ListNode;

type Number = Option<Box<ListNode<i32>>>;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Number, l2: Number) -> Number {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = ListNode::new(0);
        let mut current = &mut dummy;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut total = carry;
            if let Some(node) = l1 {
                total += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                total += node.val;
                l2 = node.next;
            }
            carry = total / 10;
            current.next = Some(Box::new(ListNode::new(total % 10)));
            current = current.next.as_mut().unwrap();
        }
        return dummy.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        linked![1, 2];
        let cases = [
            (linked![7, 0, 8], linked![2, 4, 3], linked![5, 6, 4]),
            (linked![0], linked![0], linked![0]),
            (
                linked![8, 9, 9, 9, 0, 0, 0, 1],
                linked![9, 9, 9, 9, 9, 9, 9],
                linked![9, 9, 9, 9],
            ),
        ];
        for (ans, l1, l2) in cases {
            assert_eq!(ans, Solution::add_two_numbers(l1, l2));
        }
    }
}
