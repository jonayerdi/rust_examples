mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::voice::breathe_in();
            println!("clarinet");
        }
    }
    pub mod voice {
        pub fn breathe_in() {
            println!("breathe_in");
        }
    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
        pub fn get_id(self) -> i32 {
            self.id
        }
    }
}

use self::sound::instrument;
use std::collections::HashMap;

// Rename imported types with `as`
use std::fmt::Result;
use std::io::Result as IoResult;

// External package
extern crate rand;
use rand::Rng;

fn function1() -> Result {
    Ok(())
}
fn function2() -> IoResult<()> {
    Ok(())
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();

    // Module brought to scope by `use`
    instrument::clarinet();

    // Structs
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);
    println!("The ID is {}", v.get_id());

    // HashMap
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    println!("HashMap is {:#?}", map);

    // Call functions
    function1().unwrap();
    function2().unwrap();

    // Use external package
    println!("{}", rand::thread_rng().gen_range(1, 101));
}
