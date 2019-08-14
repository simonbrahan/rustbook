use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }
}

fn main() {
    let leaf = Rc::new(Node::new(3));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("{:?}", leaf.parent.borrow().upgrade());
}
