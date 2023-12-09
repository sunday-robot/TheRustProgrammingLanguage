#![allow(unused)]

mod news_article;
mod notify;
mod summary_trait;
mod tweet;

use std::f64::consts::PI;

use crate::news_article::NewsArticle;
use crate::notify::notify;
use crate::summary_trait::SummaryTrait;
use crate::tweet::Tweet;

// Javaの場合はメンバ変数とメソッドは一つのクラスとしてまとまっているので、
// Tweetクラスをimportしていれば必然的にメソッドもアクセスできるようになるが、
// Rustでは構造体と、トレイト(関数のセット)は別々であるため、構造体だけでなく、トレイトもuseしなければならない。

pub fn main_x() {
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

/// 名前付きオブジェクト(Abstractという名前を設定しているが、Rustでは抽象クラス(抽象構造体)は定義できない。)
struct AbstractNamedObject {
    name: String,
}

impl AbstractNamedObject {
    fn new(name: String) -> AbstractNamedObject {
        AbstractNamedObject { name }
    }
}

trait AbstractNamedObjectTrait {
    fn print_name(&self);
}

impl AbstractNamedObjectTrait for AbstractNamedObject {
    fn print_name(&self) {
        println!("{:}", self.name);
    }

}

trait AbstractShapeTrait: AbstractNamedObjectTrait {
    fn area(&self) -> f64;
    fn circumference(&self) -> f64;
}

struct ShapeTraitList {
    a: Vec<Box<dyn AbstractShapeTrait>>,
}

struct CircleShape {
    base: AbstractNamedObject,
    radius: f64,
}

impl CircleShape {
    fn new(name: String, radius: f64) -> CircleShape {
        CircleShape {
            base: AbstractNamedObject::new(name),
            radius,
        }
    }
}

impl AbstractNamedObjectTrait for CircleShape {
    fn print_name(&self) {
        self.base.print_name();
    }
}

impl AbstractShapeTrait for CircleShape {
    fn area(&self) -> f64 {
        self.radius * self.radius * PI
    }

    fn circumference(&self) -> f64 {
        self.radius * 2.0 * PI
    }
}

struct RectangleShape {
    base: AbstractNamedObject,
    width: f64,
    height: f64,
}

impl RectangleShape {
    fn new(name: String, width: f64, height: f64) -> RectangleShape {
        RectangleShape {
            base: AbstractNamedObject::new(name),
            width,
            height,
        }
    }
}

impl AbstractNamedObjectTrait for RectangleShape {
    fn print_name(&self) {
        self.base.print_name();
    }
}

impl AbstractShapeTrait for RectangleShape {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn circumference(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

fn main() {
    let mut s = ShapeTraitList { a: Vec::new() };

    s.a.push(Box::new(CircleShape::new(String::from("Circle1"), 1.0)));
    s.a.push(Box::new(RectangleShape::new(
        String::from("Rectangle1"),
        2.0,
        3.0,
    )));

    for elem in &s.a {
        elem.print_name();
        println!("  area = {}", elem.area());
        println!("  circumfence = {}", elem.circumference());
    }
}
