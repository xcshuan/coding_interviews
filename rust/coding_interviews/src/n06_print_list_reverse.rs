pub struct Solution;

use super::util::linked_list;
use std::collections::VecDeque;

impl Solution {
    pub fn print_list_reverse(list: Option<Box<linked_list::ListNode>>) {
        let mut v = VecDeque::new();
        let mut temp = &list;
        while let Some(b) = temp {
            v.push_front(Some(b));
            temp = &b.next;
        }
        for (i, val) in v.iter().enumerate() {
            if let Some(b) = val {
                print!("{}", b.value);
                if i != v.len() - 1 {
                    print!("->")
                } else {
                    print!("\n")
                }
            }
        }
    }

    pub fn print_list_reverse2(list: Option<Box<linked_list::ListNode>>) {
        if let Some(b) = list {
            if b.next != None {
                Solution::print_list_reverse2(b.next);
            }
            print!("{}\t", b.value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::linked_list::ListNode;
    use super::Solution;

    #[test]
    fn test_print() {
        let a = vec![1, 2, 3, 4, 6, 3, 5];
        let l = ListNode::from_vec(a);
        Solution::print_list_reverse2(l);
    }
}
