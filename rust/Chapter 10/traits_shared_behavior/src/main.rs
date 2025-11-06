mod models;
use models::{notify, NewsArticle, Summary, Tweet};
mod partial;
use partial::Pair;

use crate::models::notify_bound;

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news = NewsArticle {
        headline: String::from("headline teste"),
        location: String::from("Location"),
        author: String::from("Autor Teste"),
        content: String::from("Conteudo Conteudo Conteudo Conteudo Conteudo "),
    };

    println!("1 new article: {}", news.summarize());

    println!("{}", tweet.test());
    println!("{}", news.test());

    println!("{:?}", notify(&tweet));
    println!("{:?}", notify(&news));

    println!("{:?}", notify_bound(&tweet));
    println!("{:?}", notify_bound(&news));

    let teste: Pair<i32> = Pair::new(32, 34);
    teste.cmp_display();

    let teste2: Pair<i32> = Pair::new(89, 34);
    teste2.cmp_display();
}
