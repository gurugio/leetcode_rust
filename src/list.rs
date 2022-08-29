pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        let n = self.head.take();
        n.map(|x| {
            self.head = x.next;
            x.elem
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|x| &mut x.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = (*node).next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn itermut() {
        let mut list: List<i32> = List::new();

        list.push(1);
        list.push(2);
        list.push(3);
        let mut itermut = list.iter_mut();
        assert_eq!(itermut.next(), Some(&mut 3));
        assert_eq!(itermut.next(), Some(&mut 2));
        assert_eq!(itermut.next(), Some(&mut 1));
        assert_eq!(itermut.next(), None);
        let mut iter2 = list.iter_mut(); // not consumed
        assert_eq!(iter2.next(), Some(&mut 3));
        assert_eq!(iter2.next(), Some(&mut 2));
        assert_eq!(iter2.next(), Some(&mut 1));
        assert_eq!(iter2.next(), None);
    }

    #[test]
    fn iter() {
        let mut list: List<i32> = List::new();

        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        let mut iter2 = list.iter(); // not consumed
        assert_eq!(iter2.next(), Some(&3));
        assert_eq!(iter2.next(), Some(&2));
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(iter2.next(), None);
    }

    #[test]
    fn intoiter() {
        let mut list: List<i32> = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        let mut intoiter = list.into_iter();
        assert_eq!(intoiter.next(), Some(3));
        assert_eq!(intoiter.next(), Some(2));
        assert_eq!(intoiter.next(), Some(1));
        assert_eq!(intoiter.next(), None);
    }

    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        assert_eq!(list.peek(), Some(&5));
        list.peek_mut().map(|x| *x = 55);
        assert_eq!(list.peek(), Some(&55));

        // Check normal removal
        assert_eq!(list.pop(), Some(55));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
