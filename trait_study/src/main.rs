#![allow(unused)]

mod notify;
mod news_article;
mod summary_trait;
mod tweet;

use crate::tweet::Tweet;
use crate::summary_trait::SummaryTrait;
use crate::news_article::NewsArticle;
use crate::notify::notify;

// Javaの場合はメンバ変数とメソッドは一つのクラスとしてまとまっているので、
// Tweetクラスをimportしていれば必然的にメソッドもアクセスできるようになるが、
// Rustでは構造体と、トレイト(関数のセット)は別々であるため、構造体だけでなく、トレイトもuseしなければならない。

pub fn main() {
    // println!("こんにちは");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("もちろん、ご存知かもしれませんがね、みなさん"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("This is NewsArticle"),
        location: String::from("Kanagawa prefecture Japan"),
        author: String::from("Mitsuhiro Akiyama"),
        content: String::from("本日は晴天なり。"),
    };

    println!("tweet.summarize(4) = {}", tweet.summarize(4));
    println!("article.summarize(8) = {}", article.summarize(8));

    println!("call notify function:");
    notify(&tweet);
}
