fn main() {
    let lyrics = [
        (5, "Five gooooold rings!"),
        (4, "Four calling birds"),
        (3, "Three french hens"),
        (2, "Two turtle doves")
    ];
    for day in 1..6 {
        println!("On day {} of christmas my true love gave to me...", day);

        for lyric in lyrics.iter() {
            let (num, text) = lyric;

            if day > num - 1 {
                println!("{}", text);
            }
        }

        if day == 1 {
            println!("A partridge in a pear tree");
        } else {
            println!("And a partridge in a pear tree");
        }
    }
}
