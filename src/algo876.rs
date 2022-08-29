// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
fn count(head: &Option<Box<ListNode>>) -> usize {
    let mut count: usize = 0;
    if head.is_some() {
        count += 1;
    }
    let mut cur = head.as_ref().and_then(|node| node.next.as_ref());
    loop {
        if let Some(node) = cur {
            count += 1;
            cur = node.next.as_ref();
        } else {
            break;
        }
    }
    count
}
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    loop {
        fast = fast.as_ref().and_then(|node| node.next.as_ref());
        if fast.is_none() {
            break;
        }
        slow = slow.as_ref().and_then(|node| node.next.as_ref());
        fast = fast.as_ref().and_then(|node| node.next.as_ref());
        if fast.is_none() {
            break;
        }
    }
    slow.map(|node| node.clone())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count() {
        let empty = None;
        assert_eq!(0, count(&empty));
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));
        assert_eq!(4, count(&head));
    }

    #[test]
    fn test_middle() {
        let head = Some(Box::new(ListNode { val: 1, next: None }));
        assert_eq!(1, middle_node(head).unwrap().val);
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        assert_eq!(2, middle_node(head).unwrap().val);

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));
        assert_eq!(3, middle_node(head).unwrap().val);
    }
}
