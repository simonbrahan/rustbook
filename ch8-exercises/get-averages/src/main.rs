use std::collections::HashMap;

#[derive(Debug)]
enum MedianVal {
    TrueValue(i32),
    MeanValue(f64),
}

fn get_mean(input: &[i32]) -> f64 {
    f64::from(input.iter().sum::<i32>()) / input.len() as f64
}

fn get_median(input: &[i32]) -> MedianVal {
    let input_length = input.len();
    let input_midpoint = (input_length as f64 / 2.0).floor() as usize;

    if input_length % 2 == 1 {
        MedianVal::TrueValue(input[input_midpoint])
    } else {
        MedianVal::MeanValue(get_mean(&input[input_midpoint..=input_midpoint]))
    }
}

fn get_mode(input: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    let mut current_highest = 0;
    let mut output = 0;

    for num in input {
        let count = counts.entry(num).or_insert(0);
        *count += 1;

        if count > &mut current_highest {
            current_highest = *count;
            output = *num;
        }
    }

    output
}

fn main() {
    let input = vec![1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("input is {:?}", input);
    println!("mean is {}: should be 5.090909090909091", get_mean(&input));
    println!("median is {:?}: should be TrueValue(5)", get_median(&input));
    println!("mode is {}: should be 1", get_mode(&input));
}
