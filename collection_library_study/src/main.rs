#![allow(unused)]

// VecやStringと異なりHashMapはuseしなければならない。
use std::collections::HashMap;

fn main() {
    study_simple_vec();
    study_string();
    study_hashmap();
}

fn study_simple_vec() {
    // イミュータブルなVecの生成
    if false {
        // イミュータブルなVecの場合は、通常は以下の様にvec!マクロで初期化する
        // (この手の機能はマクロではなく言語に組み込んで、きちんと機能を定義してほしい。マクロでの実装は「ソース読め」ほしい気がする。)
        let v = vec![2, 3, 5];

        // イミュータブルなので、値の追加はできない
        // v.push(1);

        println!("{:?}", v);
    }
    // 空のイミュータブルなVecの生成
    if false {
        // vec!マクロは空のVec生成には使用できないらしい(型が指定できないから?)

        // 以下の記述が標準的らしいが、
        let v: Vec<i32> = Vec::new();
        println!("{:?}", v);

        // 以下の記述の方がわかりやすいのではないか?
        // Javaは左辺側で型を記述するなどちょっとおかしいが、C#やkotolinなどは右辺で型を記述するようになっている。
        // 左辺等辺の両方に型を記述するというのは頭がおかしいのではないか?
        let v2 = Vec::<i32>::new();
        println!("{:?}", v2);
    }
    // ミュータブルなVecの生成、操作
    if false {
        // ベクターに要素を追加や削除する場合は、mutを付ける必要がある。
        // (なんとなくだが、mutは左辺ではなく右辺に記述する文法の方が分かりやすい気がする…)
        let mut v = Vec::<i32>::new();
        v.push(2);
        v.push(3);
        v.push(5);
        println!("{:?}", v); // "[2, 3, 5]"が表示される

        // 先頭要素の"2"を削除し、"[3, 5]"にする。
        v.remove(0);
        println!("{:?}", v);

        // 先頭要素の"3"を削除し、"[5]"にする。
        v.remove(0);
        println!("{:?}", v);

        // 先頭要素の"5"を削除し、"[]"にする。
        v.remove(0);
        println!("{:?}", v); // "[]"が表示される

        // 既に空ベクタになっているのに、さらに先頭要素を削除しようとするとプログラムが異常終了する。
        if false {
            v.remove(0);
            println!("{:?}", v);
        }

        v.clear();
        println!("{:?}", v);
    }
    // やりすぎだと思うレベルの型推論
    if false {
        // 下の行では型を右辺にも左辺にもi32という型を明記していないが、
        // その後の行でpush()の引数がi32型であるため型が推論できるということらしい。
        // (ここまでやるとコードの可読性が悪くなるように思う。)
        let mut v = Vec::new();
        v.push(2);
        v.push(3);
        v.push(5);
        println!("{:?}", v);
    }
    // 要素アクセス
    if false {
        let v = vec![1, 2, 3, 4, 5];

        // []でのアクセス
        // 記述がシンプルだが、範囲外の添え字を指定すると落ちてしまう。
        // (範囲外の添え字を絶対に使用しないことが保証されていない限り使えない?
        // もしそうだとすると、書き捨てプログラムか、絶対に自分以外に使うことがないプログラムでしか使えないということか?)
        {
            let third: &i32 = &v[2];
            println!("The third element is {}", third);

            if false {
                // 範囲外をアクセスしようとすると、プログラムが終了してしまう。
                let sixth: &i32 = &v[5];
                println!("The sixth element is {}", sixth);
            }
        }

        // get()でのアクセス
        // 範囲外の添え字を指定すると落ちはしないが、記述が非常に煩雑。
        {
            match v.get(2) {
                Some(third) => println!("The third element is {}", third),
                None => println!("There is 3rd element."),
            }
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
    // 複数の異なる方の値を保持できるVec(enumを使用したトリッキーな技)
    if false {
        // ここで示したやり方は、ベクタの要素の型がコンパイル時に確定している場合だけに使用できるものらしい。
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
        println!("{:?}", row);

        let mut row2 = Vec::<SpreadsheetCell>::new();
        row2.push(SpreadsheetCell::Int(2));
        row2.push(SpreadsheetCell::Float(3.1));
        row2.push(SpreadsheetCell::Text(String::from("ABC")));

        println!("{:?}", row2);
    }
}

fn study_string() {
    // str型とString型
    if false {
        // str。イミュータブルな文字列型(文字列定数ということでよい?)
        let data = "initial contents";

        // String。イミュータブルあるいはミュータブルな文字列型
        // (JavaやC#は、String(string)型はイミュータブルなので、RustのStringに対応するのは、JavaならStringBuffer型?(C#は知らない))
        let s1 = data.to_string();
        let s2 = "initial contents".to_string();
        let s3 = String::from("initial contents");
        println!("{} {} {}", s1, s2, s3);
    }
    // Stringへの文字列追加(おそらくRustで]"」列連結には"+"等ではなく、format!()を使用するのが通常であると思われる。)
    if false {
        let mut s1 = String::from("foo");

        // 文字列の末尾に空白文字を1つ追加する。
        s1.push(' '); // "push"よりも"append"の方が自然に思うが、C++のvectorに習ったのだろうか?
        s1.push('c');
        s1.push('h');
        s1.push('_');
        s1.push('c');
        s1.push('o');
        s1.push('n');
        s1.push('s');
        s1.push('t');
        s1.push('a');
        s1.push('n');
        s1.push('t');
        println!("s1 is {}", s1);

        // 文字列の末尾に文字列を追加する。Rustでは理由は不明だがオーバーロードができないので、"push"ではなく、"push_str"としなければならない。
        // 文字列定数の追加
        s1.push_str(" str_constant");
        // str型変数の追加
        let s2 = " str_variable";
        s1.push_str(s2);
        // String型変数の追加はできない。(理由はわからないが、追加できるようにするにはpush_string()という関数を追加しなければならないからだろうか?)
        let s3 = String::from(" string_variable");
        // s1.push_str(s3);    // str&型でないのでコンパイルエラーになる。
        s1.push_str(s3.as_str()); // as_str()すればよい。
        println!("s1 is {}", s1);
    }
    // +演算子での文字列連結
    if false {
        // 一般的な文字列連結は、Rustでは「format!()」を使うのが定石。
        // "+"演算子での文字列連結もできるのだが、RustのStringはVec<u8>のラッパーで、
        // "+"演算子はそもそもVec型のものである。"+"演算子は記述のしやすさよりも実行時効率を追求しているらしく、
        // 二つのベクタを連結して新しいベクタを返すものではなく、左のベクタの末尾に、右のベクタの要素を追加するものとなっている。
        // このような仕様であるため、文字列の連結目的としては、左のベクタの所有権が剥奪されるなど記述しにくいものになってしまっている。
        // この代替措置として"format!()"が用意されたものと思われる。

        let s1 = String::from("abc");
        let s2 = String::from("def");
        let s1s2 = format!("{}{}", s1, s2);
        println!("{}+{}={}", s1, s2, s1s2);
        // let s3 = s1 + s2.as_str();
        let s3 = s1 + &s2;
        // ↑"+"演算子は実行効率重視の仕様であるため、上記の分の結果、s1の所有権は剥奪され、s1は使用できなくなる。
        println!("={}", s3);

        // println!("s1 : {}", s1);   s1は、s3の初期化で参照されており、この時点でs1からs3に所有権が移ってしまっているのでs3の初期化以降は参照できない。
        println!("s2 : {}", s2);
    }
    // 文字列の長さ
    if false {
        // lenでの長さ取得は多くの場合、期待したものではない
        // Stringもstrもどちらの要素の型はu8で、ユニコードではなく、UTF8である。
        // このため、ASCII文字しか使用しない場合は非常に簡単であるが、漢字などを使用する場合、非常に面倒くさい。

        // 以下のlenでは、ASCII文字のみからなる文字列であるため、単純に文字数と同じ4を返す。
        let s1 = String::from("Hola");
        println!(
            "ASCII文字のみからなる文字列のバイト数は文字数に一致する：{}.len() = {}",
            s1,
            s1.len()
        );

        // 以下のlenでは、UT8エンコーディングでひらがな1文字が3バイトになるため、6を返す。
        let s1 = String::from("やあ");
        println!(
            "漢字などの場合は文字数*3になる(特殊な漢字は*3ではないかも)：{}.len() = {}",
            s1,
            s1.len()
        );
        let s1 = String::from("😀😐");
        println!(
            "絵文字の場合は文字数*4になるらしい：{}.len() = {}",
            s1,
            s1.len()
        );
    }
    // 要素へのアクセス
    if false {
        // 以下のlenでは、ASCII文字のみからなる文字列であるため、単純に文字数と同じ4を返す。
        let s1 = String::from("Hola");
        println!("ASCII文字のみからなる文字列 {} の要素アクセス", s1);
        for c in s1.chars() {
            println!("{}", c);
        }

        // // 以下のlenでは、UT8エンコーディングでひらがな1文字が3バイトになるため、6を返す。
        let s1 = String::from("やあ");
        println!("漢字などからなる文字列 {} の要素アクセス", s1);
        for c in s1.chars() {
            println!("{}", c);
        }
        let s1 = String::from("😀😐");
        println!("絵文字からなる文字列 {} の要素アクセス", s1);
        for c in s1.chars() {
            println!("{}", c);
        }
    }
}

// ハッシュマップは、言語本体に組み込みの機能ではなく標準ライブラリの一つでしかないためか、
// 便利な初期化記法などは提供されておらず、オブジェクト生成と要素の追加を通常のメソッド呼び出しで記述するしかない。
fn study_hashmap() {
    if false {
        let mut scores = HashMap::new();

        // ハッシュマップに要素を追加する。
        {
            // 以下のようには書けないのか…
            // 「scores[String::from("Blue")] = 10;」
            // 「scores["Blue"] = 10;」
            scores.insert(String::from("Blue"), 10);
            scores.insert(String::from("Yellow"), 50);
            println!("ハッシュ表scoresの内容 {:?}", scores);
        }

        // 登録済みの要素を更新する。(新規登録と同じinsert()を使用する。)
        {
            scores.insert(String::from("Blue"), 20);
            println!("ハッシュ表scoresの内容 {:?}", scores);
        }
        // 登録されている要素の個数を確認する。
        println!("要素数は {:?} です。", scores.len());
        // 要素を列挙する。
        for e in scores.iter() {
            println!("key:{:?}, value:{:?}", e.0, e.1);
        }
        // キーを列挙する。
        for e in scores.keys() {
            println!("key:{:?}, value:{:?}", e, scores.get(e));
        }
        // キーが登録済みかどうか調べる。
        {
            let blue_value = scores.get(&String::from("Blue"));
            let red_value = scores.get(&String::from("Red"));
            println!("Blue's value = {:?}", blue_value);
            println!("Red's value = {:?}", red_value);
        }
        // キーが登録済みなら何もしない未登録なら登録する。
        {
            // 以下のような面倒な記述が必要。
            scores.entry(String::from("Blue")).or_insert(123); // Blueは登録済みなので何もしない。
            scores.entry(String::from("Red")).or_insert(234); // Redは未登録なので登録される。
            println!("ハッシュ表scoresの内容 {:?}", scores);

            // entry()、or_insert()が理解的なかったので調査用のコードを書いてみた。
            {
                // Blueは登録済みなので、OccupiedEntryが返る。
                // OccupiedEntryのor_insert()は何もしないメソッドであるため、entry().or_insert()で、「登録済みなら何もしない」ということが行われる。
                let blue_entry = scores.entry(String::from("Blue"));
                println!("Blue's entry = {:?}", blue_entry);
                // 上の2文をブロックとしているのはソースコードをわかりやすくするためではなく、blue_entryの所有権を放棄するため。
                // ひとつめの文を実行すると、scoresの指すオブジェクトの所有権がblue_entryに移ってしまい、scoresが使えなくなってしまう。
            }
            {
                // Greenは未登録なので、VacantEntryが返る。
                // VacantEntryのor_insert()はinsert()を行うメソッドであるため、entry().or_insert()で、「未登録なら登録する。」ということが行われる。
                let green_entry = scores.entry(String::from("Green"));
                println!("Green's entry = {:?}", green_entry);
            }
        }
    }
    // 現在の値に基づいて更新する。
    {
        let text = "hello world wonderful world";
        let word_iterator = text.split_whitespace();

        let mut map = HashMap::new();

        for word in word_iterator {
            let count_reference: &mut i32;
            {
                // entry()は、要素が既にあればその参照を返し、なければ挿入されるべき仮想的な要素の参照を返す。
                let e = map.entry(word);
                // or_insert()は、thisが実体を持つ要素の参照であればその値の参照を返す。
                // 実体を持たない仮想的な要素の参照の場合は、引数で渡された値を持つ実態生成し、マップに追加した後、その要素の値の参照を返す。
                // ↑Rustは「ゼロコスト抽象化」なので、値0でマップに追加した後に、値を+1するなどというコードは生成せず、はなから値1でマップに追加するというコードを生成するはず。
                count_reference = e.or_insert(0);
            }
            *count_reference += 1;
        }

        println!("{:?}", map);
    }
    if false {
        let mut scores = HashMap::new();
        let key1 = String::from("Red");
        let val1 = 30;
        scores.insert(key1, val1);

        // println!("key1 : {}", key1);   // key1はCopyトレイトを実装していないString型であるため、scoreに所有権が移っているので、もうkey1は無効。
        println!("val1 : {}", val1); // val1はCopyトレイトを実装している(?)i32型であるため、scoreはval1のコピーを所有しているだけなので、val1には引き続き有効。

        // mut文字列でも、所有権が移るのは変わらない。
        let mut key2 = String::from("Green");
        let mut val2 = 50;
        scores.insert(key2, val2);
        // println!("key2 : {}", key2); // key2はCopyトレイトを実装していないString型であるため、scoreに所有権が移っているので、もうkey2は無効。
        println!("val2 : {}", val2);

        println!("scores : {:?}", scores);
    }
}

// 要素がi32などではなくオブジェクト(JavaやC#でいうとことのオブジェクト。Rustでどう呼ぶのか不明)のベクタ
fn study_objects_vec() {}
