use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct NodeNoParent {
    pub value: i32,
    pub children: RefCell<Vec<Rc<NodeNoParent>>>,
}

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}
