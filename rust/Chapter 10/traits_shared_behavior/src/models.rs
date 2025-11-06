use std::{fmt::Display, iter::Sum};

pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    fn test(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news using trait bound! {}", item.summarize())
}

// Para casos que os items possuem traits diferentes
pub fn notify_double(item1: &impl Summary, item2: &impl Summary) {}

// Para casos que seja necessario forcar o tipo da trait
// em todos os items, poderia ser usado como <T: Summary, U: OutraTrait>
pub fn notify_double_force<T: Summary>(item1: &T, item2: &T) {}

// Caso em que o item necessariamente precisa implementar
// duas traits
pub fn notify_double_trait(item: &(impl Summary + Display)) {}
pub fn notify_double_trait2<T: Summary + Display>(item: &T) {}

// Multiplas trait bounds com clausula where
pub fn some_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Summary,
{
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
        format!("the author {}", self.author)
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
