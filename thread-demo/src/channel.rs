use std::{sync::mpsc, thread};

pub fn run() {
    message_channel();
}

fn message_channel() {
    //  message channel
    let (tx, rx) = mpsc::channel();

    // move ownership to closure
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //  val is moved to tx.send() and cannot be used here
        //  println!("val is {}", val);
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    handle.join().unwrap();
}
