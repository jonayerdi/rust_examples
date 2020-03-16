struct Point {
    x: i32,
    y: i32,
}

struct Count {
    pub upper: usize,
    pub lower: usize,
    pub white: usize,
    pub punct: usize,
    pub other: usize,
}

impl Default for Count {
    fn default() -> Self {
        Count {
            upper: 0,
            lower: 0,
            white: 0,
            punct: 0,
            other: 0,
        }
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Uppercase: {}", self.upper)?;
        writeln!(f, "Lowercase: {}", self.lower)?;
        writeln!(f, "Whitespace: {}", self.white)?;
        writeln!(f, "Punctuation: {}", self.punct)?;
        write!(f, "Others: {}", self.other)
    }
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    // Match on struct
    [
        Point { x: 0, y: -7 },
        Point { x: -5, y: 2 },
        Point { x: 1, y: 0 },
    ]
    .iter()
    .for_each(|p| match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    });
    // Ignore remaining parts
    let point = Point { x: -5, y: 2 };
    match point {
        Point { x, .. } => println!("x is {}", x),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first, last: {}, {}", first, last),
    }
    // Match ranges, match guards
    let count = "Hello World!"
        .chars()
        .fold(Count::default(), |mut count, c| {
            match c {
                'A'..='Z' => count.upper += 1,
                'a'..='z' => count.lower += 1,
                ' ' | '\t' | '\n' => count.white += 1,
                _ if ['.', ',', ':', ';', '?', '!'].contains(&c) => count.punct += 1,
                _ => count.other += 1,
            }
            count
        });
    println!("{}", count);
    // Nested matching
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
    // @ Bindings
    let color = Color::Rgb(0, 80, 255);
    match color {
        Color::Rgb(_r @ 200..=255, _g @ 0..=100, _b @ 0..=100) => println!("Red"),
        Color::Rgb(_r @ 0..=100, _g @ 200..=255, _b @ 0..=100) => println!("Green"),
        Color::Rgb(_r @ 0..=100, _g @ 0..=100, _b @ 200..=255) => println!("Blue"),
        _ => println!("Other color"),
    }
}
