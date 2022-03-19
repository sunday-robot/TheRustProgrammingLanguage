// use std::sync::mpsc::Receiver;

// use std::hash::Hasher;

fn main() {
    if false {
        // 構造体を使用しない
        let width1 = 30;
        let height1 = 50;

        println!("長方形の面積は、{}平方ピクセルです", area(width1, height1));
    }

    if false {
        // tupleを使用する
        let rect2 = (30, 50);

        println!("長方形の面積は、{}平方ピクセルです", area2(rect2));
    }

    if false {
        // 構造体を使用する
        let mut rect3 = Rectangle {
            width: 30,
            height: 50,
        };

        let a3 = area3(&rect3);

        rect3.width = 1;

        let a3_2 = area3(&rect3);

        rect3.width = 2;

        println!("長方形{:#?}の面積は、{}平方ピクセルです", rect3, a3);
        println!("長方形{:?}の面積は、{}平方ピクセルです", rect3, a3_2);
    }

    if false {
        // 構造体にメソッドareaを追加し、これを使用する
        let mut rect4 = Rectangle {
            width: 30,
            height: 50,
        };

        let a4 = Rectangle::area2(&rect4);
        println!("長方形{:#?}の面積は、{}平方ピクセルです", rect4, a4);

        let a4_2 = rect4.scale(2).area();

        println!("長方形{:?}の面積は、{}平方ピクセルです", rect4, a4_2);
    }

    if true {
        // Rectangle::can_hold()を実装してみる。
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!(
            "{:?} は {:?} を内包できる : {}",
            rect1,
            rect2,
            rect1.can_hold(&rect2)
        );
        println!(
            "{:?} は {:?} を内包できる : {}",
            rect1,
            rect3,
            rect1.can_hold(&rect3)
        );
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 上のareaとは引数の構成が異なるが、rustではオーバーロードができないので、別の名前を付けている。
fn area2(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// 構造体Rect用のメソッド群
impl Rectangle {
    // 第１引数に&selfを記述することで、通常のconstメソッド定義を行う。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 非constメソッドの場合は、&selfではなく、&mut selfを第1引数に記述する。
    // 多分非constメソッドの場合は、メソッドチェインを行えるようにselfを返すのが一般的かと思う。
    fn scale(&mut self, scale: u32) -> &Rectangle {
        self.width *= scale;
        self.height *= scale;
        self
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }

    // 関連関数(Javaのstaticメソッド)も定義できる。
    // ただし、オーバーロードはできないので、area2のような別の名前にしなければならない。
    fn area2(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }
}
