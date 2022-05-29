use std::cell::RefCell;

use std::collections::VecDeque;
use std::rc::Rc;

use crate::binary_tree::tree::TreeNode;

#[allow(dead_code)]
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut left_queue = VecDeque::new();
            left_queue.push_back(root.borrow_mut().left.take());
            let mut right_queue = VecDeque::new();
            right_queue.push_back(root.borrow_mut().right.take());

            while !left_queue.is_empty() && !right_queue.is_empty() {
                let left = left_queue.pop_front().unwrap();
                let right = right_queue.pop_front().unwrap();

                if left.is_some() != right.is_some() {
                    return false;
                }

                if let (Some(left), Some(right)) = (left, right) {
                    if left.borrow().val != right.borrow().val {
                        return false;
                    }

                    left_queue.push_back(left.borrow_mut().left.take());
                    left_queue.push_back(left.borrow_mut().right.take());
                    right_queue.push_back(right.borrow_mut().right.take());
                    right_queue.push_back(right.borrow_mut().left.take());
                }
            }
        }

        true
    }
}

struct Solution {}

#[cfg(test)]
mod test {
    use std::{cell::RefCell, rc::Rc};

    use crate::binary_tree::tree::TreeNode;

    use super::Solution;

    #[test]
    fn example_1() {
        let mut one = TreeNode::new(1);
        let mut two_1 = TreeNode::new(2);
        let mut two_2 = TreeNode::new(2);
        let three_1 = TreeNode::new(3);
        let three_2 = TreeNode::new(3);
        let four_1 = TreeNode::new(4);
        let four_2 = TreeNode::new(4);

        two_1.left = Some(Rc::new(RefCell::from(three_1)));
        two_1.right = Some(Rc::new(RefCell::from(four_1)));

        two_2.left = Some(Rc::new(RefCell::from(four_2)));
        two_2.right = Some(Rc::new(RefCell::from(three_2)));

        one.left = Some(Rc::new(RefCell::from(two_1)));
        one.right = Some(Rc::new(RefCell::from(two_2)));

        let actual_result = Solution::is_symmetric(Some(Rc::new(RefCell::from(one))));
        let expected_result = true;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let mut one = TreeNode::new(1);
        let mut two_1 = TreeNode::new(2);
        let mut two_2 = TreeNode::new(2);
        let three_1 = TreeNode::new(3);
        let three_2 = TreeNode::new(3);

        two_1.right = Some(Rc::new(RefCell::from(three_1)));

        two_2.right = Some(Rc::new(RefCell::from(three_2)));

        one.left = Some(Rc::new(RefCell::from(two_1)));
        one.right = Some(Rc::new(RefCell::from(two_2)));

        let actual_result = Solution::is_symmetric(Some(Rc::new(RefCell::from(one))));
        let expected_result = false;

        assert_eq!(actual_result, expected_result)
    }
}
