fn main() {
    study_simple_vec();
    study_string();
}

fn study_simple_vec() {
    if false {
        // 以下の記述が標準的らしいが、
        let v: Vec<i32> = Vec::new();
        println!("{:?}", v);
        // 以下の記述の方がわかりやすいのではないか?
        // Javaは左辺側で型を記述するなどちょっとおかしいが、C#やkotolinなどは右辺で型を記述するようになっている。
        // 左辺等辺の両方に型を記述するというのは頭がおかしいのではないか?
        let v2 = Vec::<i32>::new();
        println!("{:?}", v2);
    }
    if false {
        // ベクターに要素を追加や削除する場合は、mutを付ける必要がある。
        let mut v = Vec::<i32>::new();
        v.push(2);
        v.push(3);
        v.push(5);
        println!("{:?}", v);

        v.remove(0);
        println!("{:?}", v);
        v.remove(0);
        println!("{:?}", v);
        v.remove(0);
        println!("{:?}", v);
        if false {
            // 要素が3つなので、4つ削除しようとするとpanicでプログラムが終了してしまう。
            v.remove(0);
            println!("{:?}", v);
        }

        v.clear();
        println!("{:?}", v);
    }
    if false {
        // 型推論：
        // 下の行では型を右辺にも左辺にもi32という型を明記していないが、
        // その後の行でpush()の引数がi32型であるため型が推論できるということらしい。(ここまでやらなくてもよいような…)
        let mut v = Vec::new();
        v.push(2);
        v.push(3);
        v.push(5);
        println!("{:?}", v);
    }
    if false {
        let v = vec![2, 3, 5];
        println!("{:?}", v);
    }
    if false {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is 3rd element."),
        }

        // 以下は存在しない6番目の要素にアクセスするので、panicでプログラムが終了してしまう。
        if false {
            let sixth: &i32 = &v[5];
            println!("The sixth element is {}", sixth);
        }

        // 以下の記述でチェックはできるが、C言語時代と同程度に記述が冗長…
        // さすがにいちいち以下のような記述はしていられない(読むだけでもうんざりする)ので、何らかモダンな書き方がなければならないが…
        match v.get(5) {
            Some(third) => println!("The 6th element is {}", third),
            None => println!("There is no 6th element."),
        }
    }
    if false {
        // ベクタとenumを使用したトリッキーな技。
        // ここで示したやり方は、ベクタの内容がコンパイル時に確定している場合だけに使用できるものらしい。
        // ベクタに複数の型のオブジェクトを入れたい場合は、通常はトレイト(Java、C#のinterfaceに相当する機能)を使うらしい。
        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        println!("[0] : {:?}", row[0]);
        println!("[1] : {:?}", row[1]);
        println!("[2] : {:?}", row[2]);
    }
}

fn study_string() {
    if false {
        let data = "initial contents";

        let s = data.to_string();
        println!("{}", s);

        // the method also works on a literal directly:
        let s = "initial contents".to_string();
        println!("{}", s);

        let s = String::from("initial contents");
        println!("{}", s);
    }
    if true {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push(' ');   // "push"よりも"append"の方が自然に思う…
        s1.push_str(s2);    // Rustではオーバーロードができないので、"push"ではなく、"push_str"としなければならない。
        println!("s1 is {}", s1);
        println!("s2 is {}", s2);
    }
    if true {

    }
}

// fn print_nth_element(v: Vec<i32>, n: i32) {
//     match v.get(n) {
//         //                      "3つ目の要素は{}です"
//         Some(third) => println!("The third element is {}", third),
//         //               "3つ目の要素はありません。"
//         None => println!("There is no third element."),
//     }
// }

// 要素がi32などではなくオブジェクト(JavaやC#でいうとことのオブジェクト。Rustでどう呼ぶのか不明)のベクタ
fn study_objects_vec() {}
