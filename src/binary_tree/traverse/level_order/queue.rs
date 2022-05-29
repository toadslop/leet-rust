use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::binary_tree::tree::TreeNode;

#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let mut result: Vec<Vec<i32>> = Vec::new();
        queue.push_back((0, root));

        while let Some((depth, node)) = queue.pop_front() {
            if let Some(node) = node {
                if let Some(level) = result.get_mut(depth) {
                    level.push(node.borrow().val);
                } else {
                    result.push(vec![node.borrow().val]);
                }

                queue.push_back((depth + 1, node.borrow_mut().left.take()));
                queue.push_back((depth + 1, node.borrow_mut().right.take()));
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

        let actual_result = Solution::level_order(Some(Rc::new(RefCell::from(one))));
        let expected_result = vec![[1], [2], [3]];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let actual_result = Solution::level_order(None);
        let expected_result: Vec<Vec<i32>> = vec![];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_3() {
        let one = TreeNode::new(1);

        let actual_result = Solution::level_order(Some(Rc::new(RefCell::from(one))));
        let expected_result = vec![[1]];

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_4() {
        let mut three = TreeNode::new(3);
        let one = TreeNode::new(1);
        let two = TreeNode::new(2);

        three.left = Some(Rc::new(RefCell::from(one)));
        three.right = Some(Rc::new(RefCell::from(two)));

        let actual_result = Solution::level_order(Some(Rc::new(RefCell::from(three))));
        let expected_result = vec![vec![3], vec![1, 2]];

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

        let actual_result = Solution::level_order(Some(Rc::new(RefCell::from(one))));
        let expected_result = vec![vec![1], vec![4, 3], vec![2]];

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

        let actual_result = Solution::level_order(Some(Rc::new(RefCell::from(three))));
        let expected_result = vec![vec![3], vec![9, 20], vec![15, 7]];

        assert_eq!(actual_result, expected_result)
    }
}
