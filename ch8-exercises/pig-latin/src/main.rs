fn main() {
    let test = String::from("The quick brown fox jumps over the lazy dog");
    let expect = String::from("He-tay uick-qay rown-bay ox-fay umps-jay over-hay he-tay azy-lay og-day");
    let actual = pig_latin(&test);

    if actual != expect {
        println!("{} is wrong", actual);
    } else {
        println!("{} is correct", actual);
    }
}

fn pig_latin(input: &str) -> String {
    let mut output_words = Vec::new();
    for word in input.split_whitespace() {
        output_words.push(pig_latin_word(&word));
    }

    output_words.join(" ")
}

fn pig_latin_word(input: &str) -> String {
    let mut output = String::new();

    if starts_with_vowel(&input) {
        output.push_str(input);
        output.push_str("-hay");
    } else {
        output.push_str(input);
    }

    output
}

fn starts_with_vowel(input: &str) -> bool {
    let maybe_first_char = input.chars().next();

    if maybe_first_char.is_none() {
        return false;
    }

    let first_char = maybe_first_char.unwrap();

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
        _ => false
    }
}
