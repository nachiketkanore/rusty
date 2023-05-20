use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn learn_multithreading() {
    // Create a channel
    let (sender, receiver) = mpsc::channel();

    // Spawn three threads as senders
    for i in 0..3 {
        let sender_thread = sender.clone();
        thread::spawn(move || {
            sender_thread.send(i).unwrap();
        });
    }

    // Spawn a receiver thread
    let receiver_thread = thread::spawn(move || {
        for received in receiver {
            println!("Received: {}", received);
        }
    });

    // Wait for all sender threads to finish
    // thread::sleep(std::time::Duration::from_secs(2));

    // Drop the sender channel to signal the receiver thread to finish
    drop(sender);

    // Wait for the receiver thread to finish
    receiver_thread.join().unwrap();
}

fn main() {
    learn_multithreading();
    // let thread1 = std::thread::spawn(|| {
    //     let mut id = 1;
    //     loop {
    //         println!("from thread 1: {}", id);
    //         std::thread::sleep(Duration::from_secs(1));
    //         id += 2;
    //     }
    // });
    // let thread2 = std::thread::spawn(|| {
    //     let mut id = 2;
    //     loop {
    //         println!("from thread 2: {}", id);
    //         std::thread::sleep(Duration::from_secs(1));
    //         id += 2;
    //     }
    // });

    // thread1.join().unwrap();
    // thread2.join().unwrap();
}
