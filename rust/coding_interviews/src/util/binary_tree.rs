use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub struct BinaryTreeNode {
    pub value: i32,
    pub left: Option<Rc<RefCell<BinaryTreeNode>>>,
    pub right: Option<Rc<RefCell<BinaryTreeNode>>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> BinaryTreeNode {
        BinaryTreeNode {
            value,
            left: None,
            right: None,
        }
    }
    pub fn set_left(mut self, left: Option<Rc<RefCell<BinaryTreeNode>>>) {
        self.left = left
    }
    pub fn set_right(mut self, right: Option<Rc<RefCell<BinaryTreeNode>>>) {
        self.right = right
    }
    pub fn print(&self) {
        if let Some(left) = &self.left {
            left.borrow().print();
        }
        print!("{}\t", self.value);
        if let Some(right) = &self.right {
            right.borrow().print();
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct BinaryTreeNodeWithParent {
    pub value: i32,
    pub parent: Option<Rc<RefCell<BinaryTreeNodeWithParent>>>,
    pub left: Option<Rc<RefCell<BinaryTreeNodeWithParent>>>,
    pub right: Option<Rc<RefCell<BinaryTreeNodeWithParent>>>,
}

impl BinaryTreeNodeWithParent {
    pub fn print(&self) {
        if let Some(left) = &self.left {
            left.borrow().print();
        }
        print!("{}\t", self.value);
        if let Some(right) = &self.right {
            right.borrow().print();
        }
    }
}
