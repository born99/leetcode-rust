// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}
#[warn(unused_mut)]
impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reverse = None;
    let mut cursor = head;

    while let Some(mut node) = cursor {
        cursor = node.next;
        node.next = reverse;
        reverse = Some(node);
    }

    reverse
}



