use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex1 = Arc::new(Mutex::new("Mutex1"));
    let mutex2 = Arc::new(Mutex::new("Mutex2"));
    let worker1 = {
        let mutex1 = mutex1.clone();
        let mutex2 = mutex2.clone();
        thread::spawn(move || {
            println!("WORKER1 thread spawned");
            let guard1 = mutex1.lock().unwrap();
            println!("WORKER1 locked {}", &guard1);
            let guard2 = mutex2.lock().unwrap();
            println!("WORKER1 locked {}", &guard2);
        })
    };
    let worker2 = {
        let mutex1 = mutex1.clone();
        let mutex2 = mutex2.clone();
        thread::spawn(move || {
            println!("WORKER2 thread spawned");
            let guard2 = mutex2.lock().unwrap();
            println!("WORKER2 locked {}", &guard2);
            let guard1 = mutex1.lock().unwrap();
            println!("WORKER2 locked {}", &guard1);
        })
    };
    worker1.join().unwrap();
    worker2.join().unwrap();
}
