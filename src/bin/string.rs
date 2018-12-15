use std::io;
use std::mem;

fn main() -> io::Result<()> {
    let str1 = String::from("你好");
    let str2 = String::from("السلام عليكم");
    let str3 = str1 + &str2; // Moves str1 into str3
    println!("{}", str3);
    println!("sizeof(char) = {}", mem::size_of::<char>());
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    Ok(())
}
