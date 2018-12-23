struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: std::fmt::Display
{
    println!("Announcement! {}", ann);
    longest(x, y)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let string1 = String::from("abcd");
    let string2;

    {
        let s: &'static str = "xyz"; // All string literals have the 'static lifetime
        string2 = s;
    }

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    println!("The longest string is still {}", 
        longest_with_an_announcement(string1.as_str(), string2, "Sample text"));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.announce_and_return_part("Bazinga"));

    println!("{}", first_word(&novel));
}
