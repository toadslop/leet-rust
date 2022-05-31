use std::cell::RefCell;

use std::collections::HashSet;
use std::rc::Rc;

use crate::binary_tree::tree::TreeNode;

impl TreeNode {
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;
        Self::subtree(root, &mut count);
        count
    }

    fn subtree(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> HashSet<i16> {
        if let Some(root) = root {
            let mut left_values = Self::subtree(root.borrow().left.clone(), count);
            let right_values = Self::subtree(root.borrow().right.clone(), count);
            left_values.extend(right_values);
            left_values.insert(root.borrow().val as i16);

            if left_values.len() == 1 {
                *count += 1;
            }

            left_values
        } else {
            HashSet::new()
        }
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
        let mut nodes: Vec<TreeNode> = [5, 1, 5, 5, 5, 5]
            .iter()
            .map(|val| TreeNode::new(*val))
            .collect();

        nodes[2].right = Some(Rc::new(RefCell::from(nodes[5].clone())));
        nodes[1].right = Some(Rc::new(RefCell::from(nodes[4].clone())));
        nodes[1].left = Some(Rc::new(RefCell::from(nodes[3].clone())));
        nodes[0].right = Some(Rc::new(RefCell::from(nodes[2].clone())));
        nodes[0].left = Some(Rc::new(RefCell::from(nodes[1].clone())));

        let expected_result = 4;

        let actual_result =
            Solution::count_unival_subtrees(Some(Rc::new(RefCell::from(nodes[0].clone()))));

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let expected_result = 0;

        let actual_result = Solution::count_unival_subtrees(None);

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_3() {
        let mut nodes: Vec<TreeNode> = [5, 5, 5, 5, 5, 5]
            .iter()
            .map(|val| TreeNode::new(*val))
            .collect();

        nodes[2].right = Some(Rc::new(RefCell::from(nodes[5].clone())));
        nodes[1].right = Some(Rc::new(RefCell::from(nodes[4].clone())));
        nodes[1].left = Some(Rc::new(RefCell::from(nodes[3].clone())));
        nodes[0].right = Some(Rc::new(RefCell::from(nodes[2].clone())));
        nodes[0].left = Some(Rc::new(RefCell::from(nodes[1].clone())));

        let expected_result = 6;

        let actual_result =
            Solution::count_unival_subtrees(Some(Rc::new(RefCell::from(nodes[0].clone()))));

        assert_eq!(actual_result, expected_result)
    }
}
