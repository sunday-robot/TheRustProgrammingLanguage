#![allow(unused)]

mod notify;
mod summary;
mod tweet;

use tweet::Tweet;
use notify::notify;

// Javaの場合はTweetクラスをimportしていれば必然的にSummarizableインターフェイスもimport済みとなるが、
// RustではTweet構造体はSummaryトレイトを含まないので、Summaryトレイトもuseしなければならない。
// SummaryトレイトをuseしたらTweet構造体もuseしたことになるということでもないので、どちらもuseしなければならない。
//use trait_study::Summary;
//use Summary;
//use trait_study::Tweet;
//use trait_study::notify;

pub fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("もちろん、ご存知かもしれませんがね、みなさん"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize(4));
    println!("{}", tweet.summarize2());

    notify(&tweet);
}
