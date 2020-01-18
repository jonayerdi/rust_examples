use std::fmt;
use std::io;
use std::io::Write;
use std::mem;
use std::ptr;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

struct Spreadsheet(Vec<SpreadsheetCell>);

impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpreadsheetCell::Int(v) => write!(f, "{}", v),
            SpreadsheetCell::Float(v) => write!(f, "{}", v),
            SpreadsheetCell::Text(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for Spreadsheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cell in &self.0 {
            write!(f, "{}\t", cell)?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Create vector
    let mut v: Vec<i32> = Vec::new();
    let mut rebuilt;
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // Pull out the various important pieces of information about `v`
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    unsafe {
        // Cast `v` into the void: no destructor run, so we are in
        // complete control of the allocation to which `p` points.
        mem::forget(v);

        // Overwrite memory, 2x values
        for i in 0..len as isize {
            ptr::write(p.offset(i), ptr::read(p.offset(i)) * 2);
        }

        // Put everything back together into a Vec
        rebuilt = Vec::from_raw_parts(p, len, cap);
    }
    // println!("{:?}", v);
    println!("{:?}", rebuilt);

    println!("{}", &rebuilt[2]);
    // println!("{}", &rebuilt[100]);
    println!("{}", rebuilt.get(100).unwrap_or_else(|| &5));

    // Change the elements of the vector
    for i in &mut rebuilt {
        *i /= 2;
    }
    for i in &rebuilt {
        print!("{} ", i);
    }
    println!("");

    // Spreadsheet
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{}", Spreadsheet(row));

    // Consuming a stack with while let
    let mut stack = vec![1, 2, 3, 4, 5, 6];
    while let Some(val) = stack.pop() {
        print!("{} ", val);
    }
    io::stdout().flush()?;

    Ok(())
}
