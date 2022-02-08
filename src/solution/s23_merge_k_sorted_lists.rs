use std::collections::BinaryHeap;
use std::cmp::{Ordering};
use std::convert::{From, Into};


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
    fn new_next(val: i32, next: Option<Box<ListNode>>) -> Self {
    ListNode {
      next,
      val
    }
  }
}

pub struct Solution{}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl From<Vec<i32>> for ListNode {
    fn from(input: Vec<i32>) -> Self {
        let mut head = ListNode::new(input[0]);
        let mut cur = &mut head;

        for i in &input[1..] {
            cur.next = Some(Box::new(ListNode::new(*i)));
            cur = cur.next.as_mut().unwrap();
        }
        head
    }
}


#[allow(dead_code)]
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for l in lists {
            if let Some(node) = l {
                heap.push(node);
            }
        }

        let mut head = ListNode::new(0);
        let mut cur= &mut head;

        while let Some(node) = heap.pop() {
            cur.next = Some(
                Box::new(
                    ListNode::new(node.val)
                )
            );
            cur = cur.next.as_mut().unwrap();
            if node.next.is_some() {
                heap.push(node.next.unwrap());
            }
        }
        head.next
    }
}

fn from(val: ListNode) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut cur = Some(&val);

    while let Some(node) = cur {
        res.push(node.val);
        cur = node.next.as_deref();
    }
    res
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use super::*;

    #[test]
    fn simple() {
        //[[1,4,5],[1,3,4],[2,6]]
        let r = Solution::merge_k_lists(
            vec![
                Some(Box::new(ListNode::from(vec![1,4,5]))),
                Some(Box::new(ListNode::from(vec![1,3,4]))),
                Some(Box::new(ListNode::from(vec![2,6]))),
            ]
        ).unwrap();
        assert_eq!(vec![1,1,2,3,4,4,5,6], from(*r));
    }
}
