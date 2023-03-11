//! 数当てゲーム

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    if false {
        // panic!()でpanic
        panic!("crash and burn");
    }
    if false {
        // ベクタの存在しない要素をアクセスするとpanic
        let v = vec![1, 2, 3];
        v[99];
    }
    // println!("{}", abc());
    if false {
        // ファイルのオープン(冗長で読みにくいコード)
        let r = File::open("hello.txt");
        let f = match r {
            Ok(file) => {
                println!("既存のファイルを読み込みオープンできました。");
                file
            }
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                // ファイルがない場合は、作成し、書き込みオープンする。(ファイルがあれば読み込みオープンなのに…このチュートリアルのサンプルプログラムは下手な上にメチャクチャ。)
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!(
                            "ファイルを新規作成し、このファイルを書き込みオープンできました。"
                        );
                        fc
                    }
                    Err(e) => {
                        println!("ファイルが作成できません。");
                        println!("理由：{:?}", e);
                        panic!()
                    }
                }
            }
            Err(error) => {
                println!("ファイルを開けません。");
                println!("理由：{:?}", error);
                panic!();
            }
        };
    }
    if false {
        // unwrap()を用いたファイルの読み込みオープン(記述が簡単になるが、結局ファイルがなければpanic!()で終了してしまうので書き捨てプログラム以外で使うことがあるのだろうか?)
        // ↑チュートリアルにその旨記述あった。アプリケーション開発では通常使わないが、以下の場合は使うことが適切であるとのこと。
        // ・試作プログラム(使い方のよくわからないサードパーティライブラリの使い方を把握するためのプログラムなど)の場合
        // ・絶対にパニックにならないことを実装者がわかっている場合(以下のコードでは、parse()が失敗しないことが分かっているので、unwrap()を使う方が実装がシンプルになってよい。)
        // use std::net::IpAddr;
        // let home: IpAddr = "127.0.0.1".parse().unwrap();
        // ・ユニットテスト(こちらはよくわかっていない)
        //
        // unwrap()は、Resultオブジェクトによる覆いを剥いでFileオブジェクトを取り出すということを意図しているものと思われるが、あまり良いとは思えない。
        // Rustに慣れているならば良いかもしれないが、そうでない場合、「ファイルをオープンして覆いをとるってどういう意味?」となってしまうと思う。
        // Perlのように"or_die()"や、"or_panic()"の方がRustに慣れていない人にも通じるのではないか?
        let f = File::open("hello.txt").unwrap();
        println!("ファイルが読み込みオープンできました。")
    }
    if false {
        // unwrap()の代わりにexpect()を用いたファイルの読み込みオープン。unwrap()との違いはエラーメッセージを指定できることのみなので、unwrap()と同じ問題を含んでいる。
        // "expect"という単語を採用した理由が全く分からない。
        let f = File::open("hello.txt").expect("hello.txtのオープンができませんでした。");
        println!("ファイルが読み込みオープンできました。")
    }
    if false {
        // "?"によるエラー委譲は関数の中でしか使えないらしい。以下は"式"の中で使おうとしているが、コンパイルエラーになってしまう。
        // let r: Result<String, io::Error> = {
        //     let mut f = File::open("hello.txt")?;
        //     let mut s = String::new();
        //     f.read_to_string(&mut s)?; // テキストファイルの内容をすべて読み、文字列変数に末尾に追加する。(既存の文字列を破棄しないことに注意。)
        //     Ok(s)
        // };
        // match r {
        //     Ok(user_name) => {
        //         println!("user name is [{}].", user_name);
        //     }
        //     Err(error) => {
        //         println!("{}", error)
        //     }
        // }
    }
    if true {
        // 通常のアプリで行うエラー処理。(いきなりプロセスが終了してしまうunwrap()やexpect()を通常のアプリケーションでは使用できない。)
        let r = read_username_from_file_2();
        match r {
            Ok(user_name) => {
                println!("user name is [{}].", user_name);
            }
            Err(error) => {
                println!("{}", error)
            }
        }
    }
    if true {}
}

// fn abc() -> &'static str {
//     "abc"
// }

// ファイルをオープンし、その内容を読む関数。
// エラー処理の移譲を使用しないで、記述したパターン。
// Javaであればファイルアクセス処理を個別にtry catchし、catch句でthrowするようなもので、書きづらく読みにくい。
fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e), // エラー発生時は呼び出し元にエラーを返す。
    };

    let mut s = String::new();

    // 上のopenと少し記述が異なるのは、openが「文」の一部で、下のread_to_stringが「式(末尾に";"がついていない)」の一部であるため。
    // Rustに慣れていないせいか、わかりづらい。
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/// ファイルをオープンし、その内容を読む関数。
/// "?"で呼び出し元にエラー処理を委譲する記述方法。
/// Javaでいえば、try catchを記述せず、メソッドの宣言部で"throws Xxx"と書くことに相当する。
/// 正常ケース(本来行いたいこと)の記述のみですむので、書きやすく、読みやすい。
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?; // テキストファイルの内容をすべて読み、文字列変数に末尾に追加する。(既存の文字列を破棄しないことに注意。)
    Ok(s)
}

// チュートリアルの初めの方に出てくる数当てゲームの改良版用のクラスGuessとのこと
// 全く改良されていない!!!

// ユーザーの入力値(変数定義)
pub struct Guess {
    value: u32,
}

// ユーザーの入力値(メソッド定義)
impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            // 予想の値は1から100の範囲でなければなりませんが、{}でした
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
