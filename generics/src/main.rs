fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > &largest {
            largest = &item;
        }
    }

    &largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));

    let character_list = vec!['r', 'g', 'y', 'h', 't', 'i', 'p'];
    println!("The largest character is {}", largest(&character_list));
}
