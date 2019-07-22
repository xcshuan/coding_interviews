pub struct Solution;
//暂未完成，应该需要用unsafe实现，以后再做

use super::util::binary_tree::BinaryTreeNodeWithParent;
use core::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryInto;
use std::mem;
use std::rc::Rc;

impl Solution {
    fn next_node_in_binary_tree(
        tnode: &BinaryTreeNodeWithParent,
    ) -> Option<Rc<RefCell<BinaryTreeNodeWithParent>>> {
        if let Some(mut right) = tnode.right.clone() {
            let mut temp_left = Some(right.clone());
            while true {
                {
                    if temp_left.clone().unwrap().try_borrow().unwrap().left == None {
                        return temp_left.clone();
                    }
                }
                temp_left = temp_left.clone().unwrap().try_borrow().unwrap().left;
            }
        }
        None
    }
}
