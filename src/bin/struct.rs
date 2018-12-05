use std::io;
use std::fmt;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
            username: {}\n\
            email: {}\n\
            sign_in_count: {}\n\
            active: {}", 
            self.username, self.email, 
            self.sign_in_count, self.active)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() -> io::Result<()> {
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::square(10);
    println!("{}", user1);
    println!("{}", user2);
    println!("{}", black);
    println!("{}", origin);
    println!("{:?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("{:?}", rect2);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );
    Ok(())
}