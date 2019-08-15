use std::thread;
use std::time::Duration;

fn main() {
    let nums = [1, 4, 5, 8, 10];
    let handle = thread::spawn(move || {
        for i in &nums {
            println!("Number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for spawned thread to finish before continuing main thread
    handle.join().unwrap();
}
