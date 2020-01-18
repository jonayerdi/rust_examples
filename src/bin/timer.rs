use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    // Create channels for sending and receieving
    let (tx, rx) = channel();

    // Spawn timer
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        tx.send("tick").unwrap();
    });

    loop {
        let reply = rx.recv();
        println!("{}", reply.unwrap());
    }
}
