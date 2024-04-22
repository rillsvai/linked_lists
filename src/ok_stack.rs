use std::mem;

pub struct SinglyLinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(SinglyLinkedList<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();

        while let Some(mut node) = current_link {
            current_link = mem::replace(&mut node.next, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    fn prepare_for_iteration() -> SinglyLinkedList<Vec<i32>> {
        let mut list = SinglyLinkedList::new();

        list.push(vec![2, 3]);
        list.push(vec![4, 5]);
        list.push(vec![2, 6]);

        list
    }
    #[test]
    fn test_basic_operations() {
        let mut list = SinglyLinkedList::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));

        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = SinglyLinkedList::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push("1".to_string());
        list.push("2".to_string());

        assert_eq!(list.peek(), Some(&"2".to_string()));
        let elem = list.peek_mut().unwrap();
        *elem = "3".to_string();
        assert_eq!(list.peek(), Some(&"3".to_string()));
    }

    #[test]
    fn test_into_iter() {
        let list = prepare_for_iteration();

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(vec![2, 6]));
        assert_eq!(iter.next(), Some(vec![4, 5]));
        assert_eq!(iter.next(), Some(vec![2, 3]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let list = prepare_for_iteration();

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&vec![2, 6]));
        assert_eq!(iter.next(), Some(&vec![4, 5]));
        assert_eq!(iter.next(), Some(&vec![2, 3]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = prepare_for_iteration();

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut vec![2, 6]));
        assert_eq!(iter.next(), Some(&mut vec![4, 5]));
        let elem = iter.next().unwrap();
        elem[0] = 100;
        assert_eq!(iter.next(), None);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&vec![2, 6]));
        assert_eq!(iter.next(), Some(&vec![4, 5]));
        assert_eq!(iter.next(), Some(&vec![100, 3]));
        assert_eq!(iter.next(), None);
    }
}
