#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn set_next(&mut self, node: Option<Box<ListNode>>) -> () {
        self.next = node;
    }
}

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = &list1;
        let mut list2 = &list2;
        let mut prehead = Some(Box::new(ListNode::new(-1)));
        let mut prev = &mut prehead;

        while list1.is_some() && list2.is_some() {
            let node1 = list1.as_ref().unwrap();
            let node2 = list2.as_ref().unwrap();
            if node1.val <= node2.val {
                prev.as_mut().unwrap().next = Some(Box::new(ListNode::new(node1.val)));
                list1 = &list1.as_ref().unwrap().next;
            } else {
                prev.as_mut().unwrap().next = Some(Box::new(ListNode::new(node2.val)));
                list2 = &list2.as_ref().unwrap().next;
            }
            prev = &mut prev.as_mut().unwrap().next;
        }

        if list1.is_some() {
            prev.as_mut().unwrap().next = list1.clone();
        }

        if list2.is_some() {
            prev.as_mut().unwrap().next = list2.clone();
        }

        prehead.unwrap().next
    }

    pub fn merge_two_lists2(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(n1), None) => Some(n1),
            (None, Some(n2)) => Some(n2),
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    Some(n1).map(|mut x| {
                        x.next = Self::merge_two_lists(x.next.take(), Some(n2));
                        x
                    })
                } else {
                    Some(n2).map(|mut x| {
                        x.next = Self::merge_two_lists(Some(n1), x.next.take());
                        x
                    })
                }
            }
            _ => None,
        }
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
        //println!("LIST 1: {:?}", list1);
        let result = Solution::merge_two_lists(list1, list2);
        println!("RESULT: {:?}", result);
        let expected = vec_to_list(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(result, expected);
    }
}
