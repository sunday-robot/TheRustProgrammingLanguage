#![allow(unused)]

// モジュールの定義
// モジュールとは、C#ならnamespaceに相当するものらしい。

// 以下のモジュール(名前空間)の定義
// crate::m1
mod m1 {
    // 公開enum
    // 外部からcrate::m1:PubEnが参照できる、各値も当然参照できる。
    pub enum PubEn {
        V1,
        V2,
    }

    // 非公開enum
    // 外部からはcrate::m1:PriEnとしても参照できないが、
    // 内部(サブモジュール含む)からは参照できる。
    enum PriEn {
        V1,
        V2,
    }

    fn cat(s1: String, s2: String) -> String {
        return format!("{}::{}", s1, s2);
    }

    pub fn module_name() -> String {
        return String::from("m1");
    }

    pub fn m1_specific_function() -> String {
        String::from("m1_specific_function")
    }

    pub mod m1_1 {
        pub fn module_name() -> String {
            // "super"は、親モジュールを示す。(ファイルシステムでいえば".."に対応する。)
            let super_module_name = super::module_name();
            // 子モジュールからは、親モジュールの"pub"のついていない関数を参照できる。
            return super::cat(super_module_name, String::from("m1_1"));
        }

        pub fn m1_1_specific_function() -> String {
            String::from("m1_1_specific_function")
        }

        fn enum_access() {
            // 公開enumは当然サブモジュールでも参照できる。
            let pub_en = super::PubEn::V1;
            // 非公開モジュールもサブモジュールからは参照できる。
            let pri_en = super::PriEn::V1;
        }

        pub mod m1_1_1 {
            pub fn module_name() -> String {
                let super_module_name = super::module_name();
                // "super::super"は、親モジュールのさらに親モジュールを示す。
                return super::super::cat(super_module_name, String::from("m1_1_1"));

                // 文字数が多くて読みにくいなあ。
            }
            pub fn m1_1_1_specific_function() -> String {
                String::from("m1_1_1_specific_function")
            }
        }

        pub mod m1_1_2 {
            pub fn module_name() -> String {
                // 親モジュールを"super"を使用せずに参照したい場合は、"crate::"でフルネームで記述する。
                let super_module_name = crate::m1::m1_1::module_name();
                return crate::m1::cat(super_module_name, String::from("m1_1_2"));

                // 文字数が多くて読みにくいなあ。
            }

            pub fn print_module_name() {
                println!("{}", module_name());
            }
            pub fn m1_1_2_specific_function() -> String {
                String::from("m1_1_2_specific_function")
            }
        }
    }
}

// チュートリアルでは以下の様にレストランを題材としてモジュール説明を行っているが、例が下手すぎる。
// レストランの接客部門など具体例を挙げない方がよいと思う。
// 少なくとも現実的なレストラン運用支援アプリなどがここで示したような設計になることはないはず。
// (SimRestaurantみたいなゲームアプリならあり得るかも?)
// 以下のことを説明しているだけなら具体例など示さず機械的にlevel_1_mod、level_2_modなどの名前でよかった。
// ・モジュールを階層的に定義できること。
// ・各モジュールに関数を定義できること。

use crate::m1::{
    m1_1::{m1_1_1::m1_1_1_specific_function, m1_1_2::m1_1_2_specific_function},
    m1_specific_function,
};

// レストランの接客部門
mod front_of_house {
    // レストランの客?
    pub mod hosting {
        // 予約待ちリストに追加する?
        fn add_to_waitlist() {}

        // 席に着く?
        fn seat_at_table() {}
    }

    // レストランの??
    pub mod serving {
        // 注文を受ける
        pub fn take_order() {
            println!("frot_of_house::serving::take_order() was called.");
        }

        // 注文された料理を出す?
        fn serve_order() {}

        // 料金を受け取る?
        fn take_payment() {}
    }
}

fn main() {
    let mn1 = m1::module_name();
    let mn2 = m1::m1_1::module_name();
    let mn3 = m1::m1_1::m1_1_1::module_name();

    println!("module name 1 = {}", mn1);
    println!("module name 2 = {}", mn2);
    println!("module name 3 = {}", mn3);

    // 相対パスで呼び出す。
    front_of_house::serving::take_order();

    // 上と同じ関数を絶対パスで呼び出す。
    crate::front_of_house::serving::take_order();

    // "crate::front..."ではなく、"::front..."と書けるかと思ったがコンパイルエラーとなってしまう。
    // また、"."ではなく"::"と２文字であるなど、C#と比べるとRustは文字数が多く、可読性がよくないように思う。

    // 公開enumはモジュール外から参照できる。
    let pub_en = crate::m1::PubEn::V1;
    // 非公開モジュールもモジュール外からは参照できない。
    // let pri_en = crate::m1::PriEn::V1;

    use_sample();
}

fn use_sample() {
    // 以下の記述で
    // use crate::m1;
    // let mn1 = module_name();
    // 以下の記述と等価になるかと思たが、そうではなかった。
    // module_nameという関数は、m1だけではなく、m1::m1_1などにも定義されているため、あいまいであるとのこと。
    // let mn1 = crate::m1::module_name();

    {
        // 最上位モジュールの場合は特殊で、useを使うことでモジュール指定なしで関数などを参照できる。
        use crate::m1;
        let m1sf = m1_specific_function();
    }
    {
        // 通常の最上位ではないモジュールの場合は、useを使うことでそのモジュール名だけで参照できる。
        use crate::m1::m1_1;
        use crate::m1::m1_1::m1_1_1;
        let m11sf = m1_1::m1_1_specific_function();
        let m111sf = m1_1_1::m1_1_1_specific_function();
    }
    {
        // 以下の様に関数名まで記述することで、最上位ではないモジュールの関数などをモジュール名を記載することなしに参照できるようになる。
        use crate::m1::m1_1::m1_1_specific_function;
        let m11sf = m1_1_specific_function();
    }
    {
        // useでモジュール名に別名を設定することができる。
        // C++、XMLにも同様な機能はあるが、相当気を付けて使用しないと、保守性を大幅に低下させてしまう危険な機能だと思われる。
        // プログラムを作るのも保守するのも使うのも本人だけでない限り、使ってはならない機能だと思う。
        // たいして便利でもないのに危険性が大きいから、JavaにもC#にもこれに対応する機能は備わっていないのだと思う。
        use crate::m1::m1_1 as X;
        use crate::m1::m1_1::m1_1_1 as Y;
        let m11sf = X::m1_1_specific_function(); // この行だけ見ても、IDEのサポートがない限りXがcrate::m1::m1_1の別名であることは全く分からない。
        let m111sf = Y::m1_1_1_specific_function();
    }
    {
        // use crate::m1::m1_1::m1_1_2 as m112;
        // let m112sf = m112::m1_1_2_specific_function();
        let m112sf = m1_1_2_specific_function();

        // println!("{}", m112sf);
    }
    {
        {
            let e = std::io::empty();
        }
        {
            // 以下のuseの書き方は、
            use std::io::{self, Write};
            let e = io::empty();
        }
        {
            // 以下の2行のuseと同じらしい。
            use std::io;
            use std::io::Write;
            let e = io::empty();
        }
    }
    {
        use crate::m1::m1_1::*;
        let s = m1_1_1_specific_function();
        println!("{}", s);
        let s2 = m1_1_2::m1_1_2_specific_function();
        println!("{}", s2);
    }
}

// 以下の記述で、crate::m1::m1_1::m1_1_2を、crate::m1_1_2という名前で外部に公開するらしい。
// 一つのものに複数の名前を付けるのはただでさえ分かりにくい(事情を知らないプログラマがソースコードの内容を理解しようとしたときに、無駄な時間を費やすことになる。)
pub use m1::m1_1::m1_1_2;

use rand::Rng;

fn rng_sample() {}
