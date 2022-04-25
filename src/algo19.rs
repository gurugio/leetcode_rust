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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut n = n;
    let mut fast = head.as_deref();
    while n > 0 && fast.is_some() {
        n -= 1;
        fast = fast.and_then(|node| node.next.as_deref());
    }

    if fast.is_none() {
        return head.and_then(|node| node.next);
    }

    let dummy: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: -1,
        next: head.clone(),
    }));
    let mut slow = dummy.as_deref();
    let mut new_dummy = dummy.clone();
    let mut cur = new_dummy.as_deref_mut();

    while fast.is_some() {
        fast = fast.and_then(|node| node.next.as_deref());
        slow = slow.and_then(|node| node.next.as_deref());

        cur.as_mut()
            .map(|node| node.next = Some(Box::new(slow.clone().unwrap().clone())));
        cur = cur.and_then(|node| node.next.as_deref_mut());
    }

    cur.as_mut().map(|node| {
        let nnext = node.next.clone().unwrap().next;
        node.next = nnext;
    });

    new_dummy.unwrap().next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove() {
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
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
            remove_nth_from_end(head.clone(), 1)
        );
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            remove_nth_from_end(head.clone(), 2)
        );
    }
}
