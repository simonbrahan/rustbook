fn main() {
    let conversions = [(10.0, 1), (32.0, 2), (48.0, 3)];

    for conversion in &conversions {
        let (num, conv_dir) = conversion;

        if *conv_dir == 1 {
            println!("{} Celsius is {} Farenheit", num, c_to_f(*num));
        } else if *conv_dir == 2 {
            println!("{} Farenheit is {} Celsius", num, f_to_c(*num));
        } else {
            println!("Don't understand the conversion {}", conv_dir);
        }
    }
}

fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
