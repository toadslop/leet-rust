use std::{borrow::BorrowMut, cmp::max};

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

    pub fn set_next(&mut self, node: Option<Box<ListNode>>) -> () {
        self.next = node;
    }
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut prehead = ListNode::new(-1);
        let mut prev = &mut prehead;
        let mut list1 = list1;
        let mut list2 = list2;

        while list1.is_some() || list2.is_some() {
            let node1 = list1.as_ref().unwrap();
            let node2 = list2.as_ref().unwrap();
            if node1.val < node2.val {
                prev.next = Some(node1.clone());
                list1 = node1.next.clone();
            } else {
                prev.next = Some(node2.clone());
                list2 = node1.next.clone();
            }
            prev = &mut *prev.next.unwrap().as_ref();
        }

        prehead.to_owned().next
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_two_sorted_lists::{ListNode, Solution};

    fn vec_to_list(mut values: Vec<i32>) -> Option<Box<ListNode>> {
        values.reverse();
        let mut current = match values.get(0) {
            Some(val) => Some(Box::new(ListNode::new(*val))),
            None => None,
        };

        for val in values[1..].iter() {
            let next = current;
            let mut temp = ListNode::new(*val);
            temp.set_next(next);
            current = Some(Box::new(temp));
        }

        current
    }

    #[test]
    fn example_1() {
        let vec1 = vec![1, 2, 4];
        let vec2 = vec![1, 3, 4];
        let list1 = vec_to_list(vec1);
        let list2 = vec_to_list(vec2);
        println!("LIST 1: {:?}", list1);
        let result = Solution::merge_two_lists(list1, list2);
        let expected = vec_to_list(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(result, expected);
    }
}
