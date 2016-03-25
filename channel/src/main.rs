use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;
            println!("send");
            tx.send(()).unwrap();
        });
    }

    println!("wait");

    for _ in 0..10 {
        println!("for wait");
        rx.recv().unwrap();
    }
}