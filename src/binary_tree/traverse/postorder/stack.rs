use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::tree::TreeNode;

#[allow(dead_code)]
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];
        let mut next = root;

        while !stack.is_empty() || next.is_some() {
            while let Some(node) = next {
                next = node.borrow_mut().left.take();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                if node.borrow().right.is_some() {
                    next = node.borrow_mut().right.take();
                    stack.push(node);
                } else {
                    result.push(node.borrow().val);
                }
            }
        }

        result
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
        let mut two = TreeNode::new(2);
        let three = TreeNode::new(3);

        two.left = Some(Rc::new(RefCell::from(three)));
        one.right = Some(Rc::new(RefCell::from(two)));

        let actual_result = Solution::postorder_traversal(Some(Rc::new(RefCell::from(one))));
        let expected_result = vec![3, 2, 1];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let actual_result = Solution::postorder_traversal(None);
        let expected_result = vec![];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_3() {
        let one = TreeNode::new(1);

        let actual_result = Solution::postorder_traversal(Some(Rc::new(RefCell::from(one))));
        let expected_result = vec![1];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_4() {
        let mut three = TreeNode::new(3);
        let one = TreeNode::new(1);
        let two = TreeNode::new(2);

        three.left = Some(Rc::new(RefCell::from(one)));
        three.right = Some(Rc::new(RefCell::from(two)));

        let actual_result = Solution::postorder_traversal(Some(Rc::new(RefCell::from(three))));
        let expected_result = vec![1, 2, 3];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_5() {
        let three = TreeNode::new(3);
        let mut one = TreeNode::new(1);
        let two = TreeNode::new(2);
        let mut four = TreeNode::new(4);

        four.left = Some(Rc::new(RefCell::from(two)));
        one.left = Some(Rc::new(RefCell::from(four)));
        one.right = Some(Rc::new(RefCell::from(three)));

        let actual_result = Solution::postorder_traversal(Some(Rc::new(RefCell::from(one))));
        let expected_result = vec![2, 4, 3, 1];

        assert_eq!(actual_result, expected_result)
    }
}
