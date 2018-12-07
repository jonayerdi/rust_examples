use std::io;

fn main() -> io::Result<()> {
    let mut stack = vec![1, 2, 3, 4, 5, 6];
    while let Some(val) = stack.pop() {
        println!("{}", val);
    }
    Ok(())
}
