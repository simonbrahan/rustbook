#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn new(items: &[i32]) -> Rc<List> {
        if items.is_empty() {
            Rc::new(List::Nil)
        } else {
            List::cons(items[0], &List::new(&items[1..]))
        }
    }

    fn cons(item: i32, list: &Rc<List>) -> Rc<List> {
        Rc::new(Cons(item, Rc::clone(&list)))
    }
}

use crate::List::Cons;
use std::rc::Rc;

fn main() {
    let a = List::new(&[5, 10]);

    println!("Count after creating a: {}", Rc::strong_count(&a));

    let b = List::cons(3, &a);

    println!("Count after creating b: {}", Rc::strong_count(&a));

    {
        let c = List::cons(4, &a);
        println!("Count after creating c: {}", Rc::strong_count(&a));
    }

    println!("Count after c leaves scope: {}", Rc::strong_count(&a));

    println!("{:?}", a);
    println!("{:?}", b);
}
