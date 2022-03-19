fn main() {
    test("hello");
    test("こ");
    test("こん");
    test("こんに");
    test("こんにち");
    test("こんにちは");
}

fn test(s: &str) {
    let s1 = String::from(s);

    let byte_count = get_byte_count(&s1);
    let length = get_char_count(&s1);

    println!(
        "'{}'の長さは {} で、バイト数は{}です。",
        s1, length, byte_count
    );

    match s1.bytes().nth(0) {
        Some(v) => println!("1バイト目は、{} です。", v),
        None => (),
    }

    if let Some(v) = s.chars().nth(0) {
        println!("1文字目は、\"{}\"です。", v)
    }
}

/// UTF8文字列のバイト数を返す。
fn get_byte_count(s: &String) -> usize {
    s.len()
}

/// 文字の個数を返す。(JavaやC#と同様なもの)
fn get_char_count(s: &String) -> usize {
    s.chars().count()
}
