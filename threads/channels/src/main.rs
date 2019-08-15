use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn block_and_wait_for_channel() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello");
        transmitter.send(val).unwrap();
    });

    let received = receiver.recv().unwrap();

    println!("Spawned thread sent \"{}\"", received);
}

fn loop_and_wait_for_channel() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello");
        thread::sleep(Duration::from_millis(1500));
        transmitter.send(val).unwrap();
    });

    while receiver.try_recv().is_err() {
        println!("Waiting for message from spawned thread");
        thread::sleep(Duration::from_millis(500));
    }

    println!("Got message from spawned thread");
}

fn send_and_receive_many_messages() {
    let (transmitter, receiver) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            thread::sleep(Duration::from_millis(500));
            transmitter.send(val).unwrap();
        }
    });

    for received_value in receiver {
        println!("message from spawned thread: \"{}\"", received_value);
    }

    println!("No more messages from spawned thread");
}

fn using_many_transmitters() {
    let (transmitter, receiver) = mpsc::channel();

    let other_transmitter = mpsc::Sender::clone(&transmitter);

    thread::spawn(move || {
        let vals = vec![
            String::from("Smart Thread: Hello"),
            String::from("Smart Thread: How are you"),
            String::from("Smart Thread: Goodbye"),
        ];

        for val in vals {
            transmitter.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Casual Thread: Yo"),
            String::from("Casual Thread: Sup"),
            String::from("Casual Thread: Laters"),
        ];

        for val in vals {
            other_transmitter.send(val).unwrap();
            thread::sleep(Duration::from_millis(400));
        }
    });

    for received_val in receiver {
        println!("Message from some thread: \"{}\"", received_val);
    }
}

fn main() {
    block_and_wait_for_channel();
    loop_and_wait_for_channel();
    send_and_receive_many_messages();
    using_many_transmitters();
}
