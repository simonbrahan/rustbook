fn main() {
    let mut greeting = String::from("Hello");

    greeting.push(' ');
    greeting.push_str("world");

    println!("{}", greeting);

    let greeting_start = String::from("hello");
    let greeting_end = String::from("world");

    println!("{}", format!("{} {}", greeting_start, greeting_end));

    for char in "नमस्ते hello!".chars() {
        println!("{}", char);
    }

    for byte in "नमस्ते".bytes() {
        println!("{}", byte);
    }
}
