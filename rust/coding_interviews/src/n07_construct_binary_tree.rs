pub struct Solution;

use super::util::binary_tree::BinaryTreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn construct_binary_tree(
        pre_order: &[i32],
        in_order: &[i32],
    ) -> Option<Rc<RefCell<BinaryTreeNode>>> {
        if (pre_order.len() != in_order.len()) || pre_order.len() == 0 {
            return None;
        }
        let root_value = pre_order[0];
        let root = Rc::new(RefCell::new(BinaryTreeNode::new(root_value)));
        if pre_order.len() == 1 {
            if pre_order[0] == in_order[0] {
                return Some(root);
            }
            return None;
        }
        if in_order[in_order.len() - 1] == root_value {
            root.borrow_mut().left = Solution::construct_binary_tree(
                &pre_order[1..pre_order.len()],
                &in_order[0..in_order.len() - 1],
            );
        } else if in_order[0] == root_value {
            root.borrow_mut().right = Solution::construct_binary_tree(
                &pre_order[1..pre_order.len()],
                &in_order[1..in_order.len()],
            );
        } else {
            for (i, v) in in_order.iter().enumerate() {
                if *v == root_value {
                    root.borrow_mut().left =
                        Solution::construct_binary_tree(&pre_order[1..i + 1], &in_order[0..i]);
                    root.borrow_mut().right = Solution::construct_binary_tree(
                        &pre_order[i + 1..pre_order.len()],
                        &in_order[i + 1..in_order.len()],
                    );
                    break;
                }
                if i >= in_order.len() - 1 {
                    return None;
                }
            }
        }
        return Some(root);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use core::borrow::BorrowMut;

    #[test]
    fn test_binary() {
        let p = [1, 4, 7, 9, 2];
        let i = [7, 4, 9, 1, 2];
        if let Some(t) = Solution::construct_binary_tree(&p, &i) {
            t.borrow().print();
            print!("\n");
        }
    }
}
