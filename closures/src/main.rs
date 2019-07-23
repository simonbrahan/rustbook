use std::thread;
use std::time::Duration;

fn main() {
    println!("Intensity will be {}", simulated_expensive_calculation(2));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
