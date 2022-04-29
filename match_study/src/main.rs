#![allow(unused)]

fn main() {
    if false {
        // 関数の中で列挙型が定義できるが、こんなスコープが狭い型を定義できても意味はないような…
        #[derive(Debug)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        // Pascalの様に関数内で関数も定義できる…
        // 10年前くらいなら意味がありそうだが、C#やJavaでもラムダ式が当たり前になりつつある現在ではあまり意味がないのではないか?
        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        fn test_value_in_cents(coin: Coin) {
            let cents = value_in_cents(coin);
            // println!("coin:{:?}", coin);
            // 上の行はコンパイルエラーになるのだが、理由が分からない。
            // エラーメッセージを見ると、Coin型がCopyトレイトを実装していないため、
            // value_in_cents()でcoinからcentへのムーブが行われてcoinという変数が無効になっているとのこと。
            println!("cents:{}", cents);
        }

        test_value_in_cents(Coin::Penny);
        test_value_in_cents(Coin::Nickel);
        test_value_in_cents(Coin::Dime);
        test_value_in_cents(Coin::Dime);
    }
    if false {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // ... などなど
        }

        #[derive(Debug)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        let coin = Coin::Quarter(UsState::Alabama);
        println!("{:?}", coin);
        let c = value_in_cents(coin);
        // let c2 = value_in_cents(coin);
        // 上の行はコンパイルエラーになる。
        // Coin型がCopyトレイトを実装していないため、一度value_in_cents()にcoin変数を渡すと、
        // 「ムーブ」が行われて、変数coinが無効になってしまうためとのこと。
        // println!("{:?}", coin);
        println!("{}", c);
    }
    if false {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
    if false {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            // ↓switch文のdefault句に相当するもの。
            _ => {
                println!("unsupported value {}", some_u8_value)
            }
        }
        let u8_5 = 5u8;
        match u8_5 {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => {}
        }
    }
    if true {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        let coin = Coin::Penny;
        let mut count = 0;
        if false {
            // else句に記述したmatch式と等価なif let式
            // match式では少し行数が多くてわかりにくいので導入されたシンタックスシュガーと思われる。
            if let Coin::Quarter(state) = coin {
                println!("State quarter from {:?}!", state);
            } else {
                count += 1;
            }
        } else {
            match coin {
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                },
                _ => {
                    count += 1;
                }
            }
        }
    }
}
