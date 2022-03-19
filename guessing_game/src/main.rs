use rand::Rng; // 標準ライブラリに乱数生成器がない(!!!)ので、外部ライブラリを使用する。
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数を当ててごらん");

    // 乱数生成器は、乱数生成メソッドgen_range()が破壊的メソッド(Ruby用語。、一般的にどう呼ばれているのかわからない)なので、mutableでなければならない。
    let mut random_generator = rand::thread_rng();

    // 数当てゲームの答え(1から100までの値)を決める。
    let secret_number = random_generator.gen_range(1, 101);

    println!("(デバッグ)答: {}", secret_number);

    loop {
        println!("1から100までの数を当ててください");

        // 変更可能な文字列型変数guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // 標準入力から1行読み、guessに代入する
            .expect("行の読み込みに失敗しました"); // 例外処理らしいが、expectというのはどういうことなのか？→第9章で説明するとのこと

        let parse_result = guess.trim().parse::<u32>();
        // ↑ "let guess:u32"として左辺で型を示すことで、"::<u32>"は省略できるが、
        // 右辺側で型を示したほうがわかりやすいように思う。
        // "parse<u32>"ではなく"parse::<u32>"でなければならない理由はわからない。

        // パース結果(32ビット整数と、エラーコードのタプル)処理
        // 同じguessという名前でu32型変数を作成(これ以降の行では文字列型のguessにはアクセスできない。シャドウィングと呼ばれる機能で
        // まさにこのような型変換処理のためにRustに用意された機能であるとのこと)
        // ただし、新規定義と再定義で記述に差異がないので、少し長いメソッドの場合、新規定義のつもりで再定義してしまうということがあり得そう。
        // シャドウィングなど無い方が良いのではないか?
        let guess = match parse_result {
            Ok(num) => num,
            Err(_) => {
                println!("不正な入力です");
                continue;
            }
        };
        // ↑matchでエラーを処理するよりも、他の言語の様に例外を投げてもらい、それをキャッチするほうがソースは見やすいと思う。

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さいです。"),
            Ordering::Equal => {
                println!("正解です。");
                break;
            }
            Ordering::Greater => println!("大きいです。"),
        }

        println!("あなたの予想: {}", guess); // : {}
    }
}

fn input_guess() -> i32 {
    // 標準入力からユーザーの入力値を読み取り、数値に変換して返すということをしたいが、
    // どう記述するのがRust流なのかわからない。
    0
}
