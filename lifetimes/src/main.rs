fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let short_string = String::from("short string");
    let longer_string = "longer string";

    let result = longest(short_string.as_str(), longer_string);

    println!("Longest string is: {}", result)
}
