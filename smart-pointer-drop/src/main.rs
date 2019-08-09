struct CustomSmartPointer {
    data: String
}

impl CustomSmartPointer {
    fn new(data: &str) -> CustomSmartPointer {
        CustomSmartPointer { data: String::from(data) }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer::new("my stuff");
    let d = CustomSmartPointer::new("other stuff");

    println!("CustomSmartPointers created");
    drop(c);
}
