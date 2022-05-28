use std::cell::RefCell;
use std::rc::Rc;

use crate::binary_tree::tree::TreeNode;

#[allow(dead_code)]
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut next = root;
        while !stack.is_empty() || next.is_some() {
            while let Some(node) = next {
                next = node.borrow_mut().left.take();
                stack.push(Some(node));
            }

            if let Some(Some(node)) = stack.pop() {
                result.push(node.borrow().val);
                next = node.borrow_mut().right.take();
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

        let actual_result = Solution::inorder_traversal(Some(Rc::new(RefCell::from(one))));
        let expected_result = vec![1, 3, 2];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let actual_result = Solution::inorder_traversal(None);
        let expected_result = vec![];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_3() {
        let one = TreeNode::new(1);

        let actual_result = Solution::inorder_traversal(Some(Rc::new(RefCell::from(one))));
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

        let actual_result = Solution::inorder_traversal(Some(Rc::new(RefCell::from(three))));
        let expected_result = vec![1, 3, 2];

        assert_eq!(actual_result, expected_result)
    }
}
