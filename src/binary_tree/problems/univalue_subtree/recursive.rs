use std::cell::RefCell;

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
        let (_, result, _) = Self::subtree(root);
        result as i32
    }

    fn subtree(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<i16>, u32, bool) {
        if let Some(root) = root {
            let (left_val, left_count, is_left_univalue) =
                Self::subtree(root.borrow().left.clone());
            let (right_val, right_count, is_right_univalue) =
                Self::subtree(root.borrow().right.clone());

            let mut new_count = left_count + right_count;
            let mut is_univalue = is_left_univalue == is_right_univalue;
            let new_val: Option<i16> = match (left_val, right_val) {
                (Some(left_val), Some(right_val)) => {
                    if left_val == right_val && root.borrow().val == left_val.into() && is_univalue
                    {
                        new_count += 1;
                        Some(left_val)
                    } else {
                        is_univalue = false;
                        Some(root.borrow().val as i16)
                    }
                }
                (Some(left_val), None) => {
                    if root.borrow().val == left_val.into() && is_univalue {
                        new_count += 1;
                        Some(left_val)
                    } else {
                        is_univalue = false;
                        Some(root.borrow().val as i16)
                    }
                }
                (None, Some(right_val)) => {
                    if root.borrow().val == right_val.into() && is_univalue {
                        new_count += 1;
                        Some(right_val)
                    } else {
                        is_univalue = false;
                        Some(root.borrow().val as i16)
                    }
                }
                (None, None) => {
                    new_count += 1;
                    Some(root.borrow().val as i16)
                }
            };

            (new_val, new_count, is_univalue)
        } else {
            (None, 0, true)
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
