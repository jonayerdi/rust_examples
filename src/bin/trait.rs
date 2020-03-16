pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    fn read_more(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Summary2 {
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary2 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Summary2: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T, U>(_t: T, _u: U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(&tweet);
    println!("{}", tweet.read_more());
    some_function(1, 2);

    let news = NewsArticle {
        headline: String::from("Coronavirus strikes"),
        location: String::from("Everywhere"),
        author: String::from("John Doe"),
        content: String::from("Lorem Ipsum"),
    };
    println!("{}", <NewsArticle as Summary>::summarize_author(&news));
    println!("{}", Summary2::summarize_author(&news));
}
