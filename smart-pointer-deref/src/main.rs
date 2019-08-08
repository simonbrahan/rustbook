use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // self is a tuple of length 1, so we need to return a reference to element 0 of self
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    let x = 5;
    let reffed_x = &x;
    let boxed_x = Box::new(x);
    let my_boxed_x = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *reffed_x);
    assert_eq!(5, *boxed_x);
    assert_eq!(5, *my_boxed_x);

    let my_name = MyBox::new(String::from("Simon"));

    hello(&my_name);
}
