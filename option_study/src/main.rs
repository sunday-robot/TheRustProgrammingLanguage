#![allow(unused)]

use core::num;
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let mut number: Option<i32> = None;

    number = Some(12);

    number += 12;

    // println!("{:+}", some_number);
}
