extern crate rand;

use std::io;
use std::io::Write; // flush()
use std::cmp::Ordering;

use rand::Rng;

const RANGE: (u32, u32) = (1, 100);

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut guess: u32;
    let secret_number = rng.gen_range(RANGE.0, RANGE.1 + 1);

    println!("Guess the number!");

    loop {
        loop {
            print!("Please input your guess {:?} -> ", RANGE);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");
            match input.trim().parse::<u32>() {
                // Ok(n) if n > 0 && n < 101 => { guess = n; break; },
                Ok(n) => {
                    match n {
                        1..=100 => { guess = n; break; },
                        _ => println!("Please type a number between {} and {}", RANGE.0, RANGE.1),
                    }
                },
                Err(_) => println!("Please type a positive integer!"),
            }
        }

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break; },
        }
    }

    Ok(())
}
