enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut list_of_ints = vec![1, 2, 3];

    let third_int = list_of_ints[2];

    println!("Third int times two is {}", times_two(third_int));

    let fourth_int = list_of_ints.get(3);

    match fourth_int {
        Some(num) => println!("Fourth int is {}", num),
        None => println!("Don't know the fourth int"),
    }

    list_of_ints.push(4);

    println!("Updated list of ints: {:?}", list_of_ints);

    for int in &mut list_of_ints {
        *int = times_two(*int);
    }

    println!("Multiplied all ints in list by two: {:?}", list_of_ints);

    let row = vec![
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Int(7),
        SpreadsheetCell::Float(3.141),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Text(val) => println!("Cell contains text: {}", val),
            SpreadsheetCell::Int(val) => println!("Cell contains integer: {}", val),
            SpreadsheetCell::Float(val) => println!("Cell contains float: {}", val),
        }
    }
}

fn times_two(num: i32) -> i32 {
    num * 2
}
