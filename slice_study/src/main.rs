fn main() {
    let s = "hello world";
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    test1("Hello, world.");
    test1("good mornig, vietnam");


    test2("Hello, world.");
    test2("good mornig, vietnam");
}

fn test1(s: &str) {
    // let mut ss = String::from(s);
    // println!("before:{}", s);
    // replace_a_to_b(s);
    // println!("after:{}", s);
    let mut ss = String::from(s);
    let index = get_first_space_index(&ss);
    ss.clear();
    println!("string = {}", s);
    println!("index = {}", index);
}

fn get_first_space_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    let bytes_iter = bytes.iter(); // 普通の要素を列挙するイテレータ
    let bytes_iter_enumerator = bytes_iter.enumerate(); // インデックスと要素のタプルを列挙するイテレータ
    for (index, &e) in bytes_iter_enumerator {
        if e == 0x20 {
            return index;
        }
    }
    s.len() // 見つからない場合は-1を返すようにしようと思ったが、usize型をi32型に変換する方法が見つけられなかった。やりたい場合はカウンタ変数を別に用意する必要がある？
}

fn test2(s: &str) {
    let mut ss = String::from(s);
    // ss.clear();ここでssを書き換えても共有されていないので問題ない。

    let first_word = get_first_word(&ss);
    // ss.clear();ssと同じバイト配列を共有するfirst_wordが生きているので、まだssを書き換えることはできない。
    println!("[{}]'s first word is [{}]", s, first_word);
    ss.clear(); // first_wordが有効なスコープではあるが、上の行以降first_wordは使用されないので、ここでssを書き換えても共有されていないので問題ない。

    let first_word2 = fake_get_first_word(&ss);
    // ss.clear();fake_get_first_word()が返すのはssとは関係のない定数なのだが、が生きているので、まだssを書き換えることはできない。
    println!("[{}]'s first word is [{}]", s, first_word2);
}

fn get_first_word(s: &String) -> &str {
    for (index, &e) in s.as_bytes().iter().enumerate() {
        if e == 0x20 {
            return &s[..index];
        }
    }
     s
}

// 偽物のget_first_word()
// 受け取った文字列とは関係のない文字列定数を返すので、
// 本関数が返す文字列を呼び出し元が保持していても、引数の文字列を書き換えるのは何ら問題ないが、
// コンパイラはそれが分からないので、引数の文字列を書き換えるコードはコンパイルエラーになる。
fn fake_get_first_word(s: &String) -> &str {
    for (index, &e) in s.as_bytes().iter().enumerate() {
        if e == 0x20 {
            return "(first word)";
        }
    }
    "(only one word)"
}

/*
fn replace_a_to_b(s: &mut str) {
    unsafe {
        let b = s.as_bytes_mut();
        for c in b.iter_mut() {
            if *c == b'a' {
                // 接頭辞"b"を文字定数の前に記述することで、u8型定数になる。ただしこれができるのはASCII文字のみで、漢字などはu8では扱えないのでコンパイルエラーになる。
                *c = b'b';
            }
        }
    }
}
*/
