fn main() {
    for num in 1..20 {
        println!("fib number {} is {}", num, fib(num));
    }
}

fn fib(num: u32) -> u32 {
    if num == 1 {
        return 0;
    }

    if num == 2 {
        return 1;
    }

    let mut last = 1;
    let mut last_but_one = 0;
    let mut count = 2;

    while count < num {
        let next = last + last_but_one;
        last_but_one = last;
        last = next;
        count += 1;
    }

    last
}
