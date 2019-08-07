use add_one;
use add_two;

fn main() {
    let num = 10;
    println!("{} plus one is {}", num, add_one::add_one(num));
    println!("{} plus two is {}", num, add_two::add_two(num));
}
