use std::collections::HashMap;

fn main() {
    // Building hashmap using ::new()
    let mut colour_scores = HashMap::new();

    colour_scores.insert(String::from("blue"), 10);
    colour_scores.insert(String::from("yellow"), 50);

    dbg!(&colour_scores);

    // Building hashmap by zipping two vectors
    let teams = vec![String::from("Rebellion"), String::from("Empire")];
    let scores = vec![15, 50];

    let team_scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    dbg!(&team_scores);

    // Accessing hash map item
    println!("rebels scored {}", team_scores[&String::from("Rebellion")]);

    // Iterating over a hash map - no guaranteed order
    for (colour, score) in &colour_scores {
        println!("{} scored {}", colour, score);
    }

    // Overwriting a value
    let mut words = HashMap::new();
    words.insert(
        String::from("label"),
        String::from("what something is called"),
    );

    dbg!(&words);

    words.insert(
        String::from("label"),
        String::from("what something is labelled"),
    );

    dbg!(&words);

    // Inserting if no value
    words
        .entry(String::from("label"))
        // "|| func" is a closure with zero arguments.
        // The book suggests .or_insert(String::from(...))
        // But this will always resolve String::from(...).
        // Using or_insert_with() allows a closure to be specified,
        // which defers the call until it is required.
        .or_insert_with(|| String::from("what we label something"));
    words
        .entry(String::from("age"))
        .or_insert_with(|| String::from("how old something is"));

    dbg!(&words);

    // Updating existing values
    let sentence = "hello oh world oh what a wonderful world";
    let mut word_counts = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }

    dbg!(&word_counts);
}
