#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(edge: u32) -> Rectangle {
        Rectangle {
            width: edge,
            height: edge,
        }
    }

    fn can_hold(&self, candidate: &Rectangle) -> bool {
        self.height >= candidate.height && self.width >= candidate.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle::square(60);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
