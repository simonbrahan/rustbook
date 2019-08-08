#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new(items: &[i32]) -> List {
        if items.is_empty() {
            List::Nil
        } else {
            List::Cons(items[0], Box::new(List::new(&items[1..])))
        }
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let other_list = List::new(&[1, 2, 3]);

    println!("{:?}", list);
    println!("{:?}", other_list);
}
