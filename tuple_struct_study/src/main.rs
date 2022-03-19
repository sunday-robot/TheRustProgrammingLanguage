// タプル構造体
// フィールドに名前を付けるのが面倒な場合に使用する？
// どのような場合に有用なのかわからない。

fn main() {
    println!("Hello, world!");

    let black = Color(1, 2, 3);

    println!("({},{},{})", black.0, black.1, black.2);
}

struct Color(i32, i32, i32);
