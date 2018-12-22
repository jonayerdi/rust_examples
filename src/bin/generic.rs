extern crate rand;

use std::mem;
use std::cmp::PartialOrd;
use rand::prelude::*;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn largest<T: Copy + PartialOrd>(list: &[T]) -> Option<T> {
    if list.len() < 1 {
        return None;
    }
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut list: [u32; 20] = unsafe { mem::uninitialized() };
    for i in 0..list.len() {
        list[i] = rng.gen_range(0, 100);
    }
    println!("{:?}", Point { x: 5, y: 6 });
    println!("{:?}", Point { x: 1.32, y: 0.5 });
    println!("{:?}", list);
    println!("Largest: {:?}", largest(&list));
}
