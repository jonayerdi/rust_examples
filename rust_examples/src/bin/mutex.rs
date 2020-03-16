use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn worker(
    mutex: Arc<Mutex<usize>>,
    name: &'static str,
    deadline: Instant,
    period: Duration,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        while Instant::now() < deadline {
            {
                let mut data = mutex.lock().unwrap();
                println!("{}: {}", name, data);
                *data += 1
            }
            thread::sleep(period);
        }
    })
}

fn main() {
    let deadline = Instant::now() + Duration::from_secs(3);
    let mutex = Arc::new(Mutex::new(1));
    let hurr = worker(mutex.clone(), "hurr", deadline, Duration::from_millis(23));
    let durr = worker(mutex, "durr", deadline, Duration::from_millis(32));
    hurr.join().unwrap();
    durr.join().unwrap();
}
