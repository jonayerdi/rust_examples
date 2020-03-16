use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello World!").unwrap();
    })
    .join()
    .unwrap();
    // After sender is dropped, buffered messages can still be received
    println!("{}", rx.recv().unwrap());
    // RecvError, since there are no more buffered messages and sender is dropped
    assert_eq!(rx.recv(), Err(mpsc::RecvError));
}
