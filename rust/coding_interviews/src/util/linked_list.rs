#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> ListNode {
        ListNode {
            value: val,
            next: None,
        }
    }
    pub fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = current;
            current = Some(Box::new(node));
        }
        current
    }
}
