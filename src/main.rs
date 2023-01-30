#![allow(unused_must_use)]
use std::io::{stdout, Write};
use rdev::listen;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // spawn new thread because listen blocks
    let (schan, rchan) = channel();
    let _listener = thread::spawn(move || {
        listen(move |event| {
            schan
                .send(event)
                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
        })
        .expect("Could not listen");
    });

    let mut lock = stdout().lock();

    let mut events = Vec::new();
    for event in rchan.iter() {
        write!(lock, "Received {:?}\n", event);
        lock.flush();
        events.push(event);
    }
}
