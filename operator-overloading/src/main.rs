use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug, PartialEq)]
struct Millimetres(u32);

struct Metres(u32);

// "Add" specifies a default generic type parameter of "self"
// so the type declaration is not needed.
impl Add for Millimetres {
    type Output = Millimetres;

    fn add(self, other: Millimetres) -> Millimetres {
        Millimetres(self.0 + other.0)
    }
}

// Default generic types can be overridden if needed, as shown here.
impl Add<Metres> for Millimetres {
    type Output = Millimetres;

    fn add(self, other: Metres) -> Millimetres {
        Millimetres(self.0 + other.0 * 1000)
    }
}

fn main() {
    assert_eq!(Point::new(1, 0) + Point::new(2, 3), Point::new(3, 3));

    assert_eq!(Millimetres(5) + Metres(1), Millimetres(1005));

    assert_eq!(Millimetres(5) + Millimetres(5), Millimetres(10));
}
