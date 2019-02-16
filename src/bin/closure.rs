use std::collections::HashMap;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: HashMap<u32,u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if let Some(value) = self.values.get(&arg) {
            *value
        } else {
            let value = (self.calculation)(arg);
            self.values.insert(arg, value);
            value
        }
    }
}

fn main() {
    let mut factorial = Cacher::new(|x| {
        println!("Calculating for {}...", x);
        let mut y = 1;
        for i in 1..=x {
            y *= i;
        }
        y
    });
    for i in 0..10 {
        println!("factorial({}) = {}", i, factorial.value(i));
    }
    for i in 0..12 {
        println!("factorial({}) = {}", i, factorial.value(i));
    }
    // FnOnce
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
