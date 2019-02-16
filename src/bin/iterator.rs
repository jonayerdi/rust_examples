use std::iter::{Iterator,FromIterator};

struct BitsIterator<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> BitsIterator<'a> {
    fn new(bytes: &'a [u8]) -> BitsIterator<'a> {
        BitsIterator { bytes, position: 0 }
    }
}

impl<'a> Iterator for BitsIterator<'a> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.position < self.bytes.len()*8 {
            let bit = self.bytes[self.position/8] & 1<<(self.position%8);
            self.position += 1;
            match bit {
                0 => Some(false),
                _ => Some(true),
            }
        } else {
            None
        }
    }
}

fn main() {
    let bits = BitsIterator::new(&[1,2,3]);
    for bit in bits {
        print!("{}", if bit {1} else {0});
    }
    println!("");
    // FromIterator, map()
    let bits = BitsIterator::new(&[1,2,3]);
    let bits_vector = Vec::from_iter(bits.map(|bit| if bit {1} else {0}));
    println!("{:?}", bits_vector);
    // filter()
    let numbers: Vec<u32> = (1..=100).filter(|n| n%2 == 0).collect();
    println!("{:?}", numbers);
}
