use std::{fmt::Debug, iter::FromIterator, ptr::null_mut};

#[derive(Debug)]
pub struct List<T>
where
    T: Debug,
{
    head: Link<T>,
    tail: *mut Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T>
where
    T: Debug,
{
    pub data: T,
    next: Link<T>,
}

impl<T> Node<T>
where
    T: Debug,
{
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

    pub fn push(&mut self, data: T) {
        let mut node = Node::new(data);
        node.next = self.next.take();
        self.next = Some(Box::new(node));
    }

    pub fn replace(&mut self, data: T) {
        self.data = data;
    }

    pub fn replace_with_list(&mut self, list: List<T>) {
        let next = self.next.take();
        if let Some(node) = list.head {
            self.next = node.next;
            self.data = node.data;
            unsafe { list.tail.as_mut() }
                .map(|node| node.as_mut())
                .flatten()
                .map(|node| node.next = next);
        }
    }

    pub fn insert(&mut self, list: List<T>) {
        let next = self.next.take();
        self.next = list.head;
        unsafe { list.tail.as_mut() }
            .map(|node| node.as_mut())
            .flatten()
            .map(|node| node.next = next);
    }

    fn is_eq(left: &Box<Node<T>>, right: &Box<Node<T>>) -> bool {
        let left: *const Node<T> = left.as_ref();
        let right: *const Node<T> = right.as_ref();
        left == right
    }
}

impl<T> List<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: null_mut(),
        }
    }

    pub fn push(&mut self, data: T) {
        let node = Some(Box::new(Node::new(data)));
        match unsafe { self.tail.as_mut() } {
            Some(Some(tail)) => {
                tail.next = node;
                self.tail = &mut tail.next;
            }
            _ => {
                self.head = node;
                self.tail = &mut self.head;
            }
        };
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        Some(node.data)
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        match (&mut self.head, unsafe { self.tail.as_mut() }?) {
            (Some(head), Some(tail)) if Node::is_eq(head, tail) => {
                self.head.take().map(|node| node.data)
            }
            (head, Some(tail)) => {
                let mut node = head;
                let mut prev_node = null_mut();
                loop {
                    prev_node = node;
                    match node {
                        Some(inner) => {
                            node = &mut inner.next;
                            let is_eq = node
                                .as_ref()
                                .map(|node| Node::is_eq(node, tail))
                                .unwrap_or(true);
                            if is_eq {
                                break;
                            };
                        }
                        None => break,
                    }
                }
                let data = unsafe { self.tail.as_mut()? }.take().map(|node| node.data);
                self.tail = prev_node;
                data
            }
            _ => None,
        }
    }

    pub fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter { link: &self.head }
    }

    pub fn iter_mut<'a>(&'a mut self) -> ListMutIter<'a, T> {
        ListMutIter {
            link: self.head.as_mut().map(|node| node.as_mut()),
        }
    }
}

#[derive(Debug)]
pub struct ListIter<'a, T>
where
    T: Debug,
{
    link: &'a Link<T>,
}

impl<'a, T> Iterator for ListIter<'a, T>
where
    T: Debug,
{
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.link.as_ref()?;
        self.link = &node.next;
        Some(node.as_ref())
    }
}

#[derive(Debug)]
pub struct ListIntoIter<T>
where
    T: Debug,
{
    link: Link<T>,
}

impl<T> IntoIterator for List<T>
where
    T: Debug,
{
    type Item = T;

    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter { link: self.head }
    }
}

impl<T> Iterator for ListIntoIter<T>
where
    T: Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.link.take()?;
        self.link = node.next;
        Some(node.data)
    }
}

#[derive(Debug)]
pub struct ListMutIter<'a, T>
where
    T: Debug,
{
    link: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for ListMutIter<'a, T>
where
    T: Debug,
{
    type Item = &'a mut Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let link = self.link.take()?;
        let next = link.next.as_mut().map(|node| node.as_mut());
        if let Some(next) = next {
            let next: *mut Node<T> = next;
            self.link = unsafe { next.as_mut() };
        }
        Some(link)
    }
}

impl<T> FromIterator<T> for List<T>
where
    T: Debug,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = List::new();
        for elem in iter {
            list.push(elem);
        }
        list
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn iter() {
        let list: List<u32> = (0..10).collect();
        assert_eq!(list.iter().count(), 10);
        assert_eq!(
            list.into_iter().collect::<Vec<u32>>(),
            (0..10).collect::<Vec<u32>>()
        );
    }

    #[test]
    pub fn iter_mut() {
        let mut list: List<u32> = (0..10).collect();
        for elem in list.iter_mut() {
            elem.push(elem.data);
        }
        assert_eq!(list.iter().count(), 20);
    }

    #[test]
    pub fn pop_tail() {
        let mut list: List<u32> = (0..10).collect();
        while let Some(elem) = list.pop_tail() {
            dbg!(elem);
        }
    }
}
