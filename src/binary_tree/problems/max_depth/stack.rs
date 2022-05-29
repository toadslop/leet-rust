use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

use crate::binary_tree::tree::TreeNode;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut stack = vec![];
        if let Some(root) = root {
            stack.push((root, 1));
        }

        while let Some((node, depth)) = stack.pop() {
            result = max(result, depth);
            if let Some(left) = node.borrow_mut().left.take() {
                stack.push((left, depth + 1));
            }
            if let Some(right) = node.borrow_mut().right.take() {
                stack.push((right, depth + 1));
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

        let actual_result = Solution::max_depth(Some(Rc::new(RefCell::from(one))));
        let expected_result = 3;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let actual_result = Solution::max_depth(None);
        let expected_result = 0;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_3() {
        let one = TreeNode::new(1);

        let actual_result = Solution::max_depth(Some(Rc::new(RefCell::from(one))));
        let expected_result = 1;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_4() {
        let mut three = TreeNode::new(3);
        let one = TreeNode::new(1);
        let two = TreeNode::new(2);

        three.left = Some(Rc::new(RefCell::from(one)));
        three.right = Some(Rc::new(RefCell::from(two)));

        let actual_result = Solution::max_depth(Some(Rc::new(RefCell::from(three))));
        let expected_result = 2;

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

        let actual_result = Solution::max_depth(Some(Rc::new(RefCell::from(one))));
        let expected_result = 3;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_6() {
        let mut three = TreeNode::new(3);
        let nine = TreeNode::new(9);
        let mut twenty = TreeNode::new(20);
        let fifteen = TreeNode::new(15);
        let seven = TreeNode::new(7);

        twenty.left = Some(Rc::new(RefCell::from(fifteen)));
        twenty.right = Some(Rc::new(RefCell::from(seven)));
        three.left = Some(Rc::new(RefCell::from(nine)));
        three.right = Some(Rc::new(RefCell::from(twenty)));

        let actual_result = Solution::max_depth(Some(Rc::new(RefCell::from(three))));
        let expected_result = 3;

        assert_eq!(actual_result, expected_result)
    }
}
