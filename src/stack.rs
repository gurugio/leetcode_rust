use std::rc::Rc;

pub struct List<T: std::fmt::Debug> {
    head: Option<Rc<Node<T>>>,
}

pub struct Node<T: std::fmt::Debug> {
    elem: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: std::fmt::Debug> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn scan(&self) {
        let mut cur = &self.head;
        println!("----");
        loop {
            match cur {
                None => break,
                Some(n) => {
                    println!("{:?}", n.elem);
                    cur = &n.next;
                }
            }
        }
        println!("----");
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|n| n.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.elem)
    }
}

pub struct Iter<'a, T: std::fmt::Debug> {
    next: Option<&'a Node<T>>,
}

impl<T: std::fmt::Debug> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T: std::fmt::Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn iter() {
        let list: List<i32> = List::new();
        let list = list.prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn basics() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
/*
impl<T: std::fmt::Debug> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take(); // move out
        println!("----");
        loop {
            if cur.is_none() {
                break;
            } else {
                println!("drop: {:?}", cur.as_ref().map(|e| e.elem));
                cur = cur.unwrap().next;
            }
        }
        println!("----");
    }
}
*/
