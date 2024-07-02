use crate::list::model::{ListNumber, Node};
use crate::List;
use std::rc::Rc;

impl List<ListNumber> {
    pub fn new() -> Self {
        List {
            head: None,
            index: 0,
        }
    }

    pub fn head(&self) -> Option<&ListNumber> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            index: 0,
        }
    }

    pub fn push_front(&self, elem: ListNumber) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
            index: 0,
        }
    }

    pub fn push_back(&self, elem: ListNumber) -> Self {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
            index: 0,
        }
    }

    pub fn iter(&self) -> Iter<'_, ListNumber> {
        Iter {
            next: self.head.as_deref(),
            index: self.index,
        }
    }

    pub fn drop_list(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, ListNumber> {
    next: Option<&'a Node<ListNumber>>,
    index: usize,
}

impl<'a, ListNumber: std::fmt::Debug> Iterator for Iter<'a, ListNumber> {
    type Item = &'a ListNumber;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            println!("\n &node.elem : {:#?}", &node);
            &node.elem
        })
    }

    fn count(self) -> usize {
        self.fold(0, |count, _| count + 1)
    }
}
