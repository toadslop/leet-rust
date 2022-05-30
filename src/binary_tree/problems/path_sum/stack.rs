use std::cell::RefCell;

use std::rc::Rc;

use crate::binary_tree::tree::TreeNode;

// impl TreeNode {
//     pub fn is_leaf(&self) -> bool {
//         self.left.is_none() && self.right.is_none()
//     }
// }

#[allow(dead_code)]
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let sum = node.borrow().val;
            let mut stack = vec![(sum, node)];

            while let Some((sum, node)) = stack.pop() {
                if node.borrow().is_leaf() && sum == target_sum {
                    return true;
                }

                if let Some(left) = node.borrow_mut().left.take() {
                    let sum = sum + left.borrow().val;
                    stack.push((sum, left));
                }

                if let Some(right) = node.borrow_mut().right.take() {
                    let sum = sum + right.borrow().val;
                    stack.push((sum, right));
                }
            }
        }

        false
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
        let mut nodes: Vec<TreeNode> = [5, 4, 8, 11, 13, 4, 7, 2, 1]
            .iter()
            .map(|val| TreeNode::new(*val))
            .collect();

        nodes[5].right = Some(Rc::new(RefCell::from(nodes[8].clone())));
        nodes[3].right = Some(Rc::new(RefCell::from(nodes[7].clone())));
        nodes[3].left = Some(Rc::new(RefCell::from(nodes[6].clone())));
        nodes[2].right = Some(Rc::new(RefCell::from(nodes[5].clone())));
        nodes[2].left = Some(Rc::new(RefCell::from(nodes[4].clone())));
        nodes[1].left = Some(Rc::new(RefCell::from(nodes[3].clone())));
        nodes[0].right = Some(Rc::new(RefCell::from(nodes[2].clone())));
        nodes[0].left = Some(Rc::new(RefCell::from(nodes[1].clone())));

        let target_sum = 22;

        let actual_result =
            Solution::has_path_sum(Some(Rc::new(RefCell::from(nodes[0].clone()))), target_sum);
        let expected_result = true;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_2() {
        let mut nodes: Vec<TreeNode> = [1, 2, 3].iter().map(|val| TreeNode::new(*val)).collect();

        nodes[0].right = Some(Rc::new(RefCell::from(nodes[2].clone())));
        nodes[0].left = Some(Rc::new(RefCell::from(nodes[1].clone())));

        let target_sum = 5;

        let actual_result =
            Solution::has_path_sum(Some(Rc::new(RefCell::from(nodes[0].clone()))), target_sum);
        let expected_result = false;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_3() {
        let target_sum = 5;

        let actual_result = Solution::has_path_sum(None, target_sum);
        let expected_result = false;

        assert_eq!(actual_result, expected_result)
    }

    #[test]
    fn example_4() {
        let target_sum = 0;

        let actual_result = Solution::has_path_sum(None, target_sum);
        let expected_result = false;

        assert_eq!(actual_result, expected_result)
    }
}
