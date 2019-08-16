use std::sync::{Arc, Mutex};
use std::thread;

fn mutex_in_single_thread() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m is now \"{:?}\"", m);
}

fn mutex_in_many_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter is at {}", *counter.lock().unwrap());
}

fn main() {
    mutex_in_single_thread();
    mutex_in_many_threads();
}
