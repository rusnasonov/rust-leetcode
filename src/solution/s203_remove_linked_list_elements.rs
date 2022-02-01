pub struct Solution {}


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  val: i32,
  next: Option<Box<ListNode>>
}

#[allow(dead_code)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[allow(dead_code)]
impl Solution {
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        loop {
            match *cur {
                None => break,
                Some(ref mut node) if node.val == val => {
                    *cur = node.next.take();
                },
                Some(ref mut node) => {
                    cur = &mut node.next;
                }
            }
        }
        head
    }
}
