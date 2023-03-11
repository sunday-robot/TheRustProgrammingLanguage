fn main_2() {
    let result : &str;
    let s1 = String::from("abc");
    {
        let s2 = "xyzAB";
        result = longest(s1.as_str(), s2);
        println!("The longest string is {}", result);
        println!("s2 = {}", s2);
    }
    println!("The longest string is {}", result);
    println!("s1 = {}", s1);
}

/// 2つの文字列のうち、長い方の文字列を返すというつまらない関数。
/// ライフタイムの説明のためのもの。
/// aは、ライフタイムの名前(予約語ではなくbやabcでも同じ。多分実質的に名前が必要ないときに慣習としてaとしているだけと思われる。
/// 意味が分からないのは、以下の記述ではxとyに同じライフタイムを指定している点で、
/// xとyのライフタイムが異なっていたらコンパイルエラーになるのではないかと思われたが、そのようなことはなかった。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
