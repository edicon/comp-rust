// Async Programming in Rust — Part 1: Threads and Channels
// -https://medium.com/@KevinBGreene/async-programming-in-rust-part-1-threads-and-channels-736f8c87b04e
use std::{thread, thread::JoinHandle, time::Duration};
use rand::{self, Rng};

mod channel;

fn main() {
    let handle: JoinHandle<i32> = thread::spawn(|| {
        let mut rng = rand::thread_rng();
        let sleep_time = rng.gen_range(1..10);
        println!("Sleeping for {} seconds", sleep_time);
        thread::sleep(Duration::from_secs(sleep_time));
        println!("Done sleeping");
        5
    });
    println!("Waiting and Retunr: {}", handle.join().unwrap());

    make_threads();
    fork_join();

    channel::run();
}

fn make_threads() {
    //  make sample thread
    let mut handles = vec![];
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let sleep_time = rand::thread_rng().gen_range(1..=10);
            println!("sleeping for {} seconds", sleep_time);
            thread::sleep(Duration::from_secs(sleep_time));
            println!("done sleeping");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

fn fork_join()  {
    let first_handle = thread::spawn(|| {
        let sleep_time = rand::thread_rng().gen_range(1..=10);
        println!("first: sleeping for {} seconds", sleep_time);
        thread::sleep(Duration::from_secs(sleep_time));
        println!("first: done sleeping");
        "first"
    });
    let second_handle = thread::spawn(|| {
        let sleep_time = rand::thread_rng().gen_range(1..=10);
        println!("second: sleeping for {} seconds", sleep_time);
        thread::sleep(Duration::from_secs(sleep_time));
        println!("second: done sleeping");
        "second"
    });

    let name = format !(
        "{} and {}",
        first_handle.join().unwrap(),
        second_handle.join().unwrap()
    );
    println!("name: {}", name);
}
