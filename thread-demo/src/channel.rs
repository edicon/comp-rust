use std::{sync::mpsc, thread};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use rand::{self, Rng};

pub fn run() {
    message_channel();
    iterator_receiver();
    sharing_receiver();
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

fn iterator_receiver() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for val in rx {
            println!("Got: {}", val);
        }
    });

    for i in 0..10 {
        tx.send(i).unwrap();
        thread::sleep(Duration::from_millis(500));
    }
}

fn sharing_receiver() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    for id in 0..4 {
      let rx = Arc::clone(&rx);
      thread::spawn(move || loop {
        let val = rx.lock().unwrap().recv().unwrap();
        println!("val = {}, in thread-{}", val, id);
      });
    };

    for i in 0..10 {
        tx.send(i).unwrap();
        let delay = rand::thread_rng().gen_range(1..=1000);
        thread::sleep(Duration::from_millis(delay));
    }
}
