#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

impl Shoe {
    fn new(size: u32, style: &str) -> Shoe {
        Shoe {
            size,
            style: String::from(style),
        }
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let sum: i32 = v1.iter().sum();
    assert_eq!(sum, 6);

    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|num| num + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn can_filter_shoes() {
    let shoes = vec![
        Shoe::new(10, "sneaker"),
        Shoe::new(13, "sandal"),
        Shoe::new(10, "boot"),
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![Shoe::new(10, "sneaker"), Shoe::new(10, "boot"),]
    );
}

#[test]
fn counter_counts_to_5() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
