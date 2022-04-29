#![allow(unused)]

use core::num;
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let mut number: Option<i32> = None;

    number = Some(12);
    print_option(number);

    number = None;
    print_option(number);
}

fn print_option(x:Option<i32>) {
    // Option型の変数から値を取り出すのは、以下のように少し煩雑な記述が必要らしい。
    if let Some(v) = x {
        println!("{}", v);
    } else {
        println!("<None>");
    }
}
