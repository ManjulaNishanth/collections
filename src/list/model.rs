use std::rc::Rc;

pub type ListNumber = i32;

#[derive(Debug)]
pub struct List<ListNumber> {
    pub head: Link<ListNumber>,
    pub index: usize,
}

type Link<ListNumber> = Option<Rc<Node<ListNumber>>>;

#[derive(Debug)]
pub struct Node<ListNumber> {
    pub elem: ListNumber,
    pub next: Link<ListNumber>,
}
