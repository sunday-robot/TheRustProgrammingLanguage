// /// T型のリストから、最大値を見つけ、返す。
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     // i32型配列内の最大値を表示する。
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     // char型配列内の最大値を表示する。
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

#[allow(dead_code)]

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// 二つの異なる方の値を持つもの。(チュートリアルではPointという名前で定義していたが、いくらチュートリアルとはいえこのひどすぎる名前なのでPairとした。Tupleの方が普通かも。)
struct Pair<T1:Clone, T2:Clone> {
    f1: T1,
    f2: T2,
}

impl<T1:Clone, T2:Clone> Pair<T1, T2> {
    // selfのf1と、引数otherのf2からなる新たなPairを返す。
    // ジェネリクスを説明することだけが目的の、おかしなメソッド。
    // selfのPairと、otherのPairは各要素の型が異なっていて良い。
    fn mixup<T3:Clone, T4:Clone>(&self, other: &Pair<T3, T4>) -> Pair<T1, T4> {
        Pair {
            f1: self.f1.clone(),
            f2: other.f2.clone(),
        }
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    if false {
        let ip = Point { x: 5, y: 10 };
        let dp = Point { x: 1.2, y: 4.3 };
        println!("{:?}", ip);
        println!("{:?}", dp);
    }
    if false {
        let opt_some = Option::Some(1);
        let opt_none: Option<i32> = Option::None;
        println!("some={:?}", opt_some);
        println!("none={:?}", opt_none);
    }
    if false {
        let ip = Point { x: 5, y: 10 };
        let dp = Point { x: 1.2, y: 4.3 };
        println!("ip.x() = {}", ip.x());
        println!("dp.x() = {}", dp.x());
    }
    if true {
        // 構成要素の型が異なる二つのPairオブジェクト
        let p1 = Pair { f1: 5, f2: 10.4 };
        let p2 = Pair { f1: "Hello", f2: 'c' };
        let p3 = p1.mixup(&p2);

        println!("p1 = ({}, {})", p1.f1, p1.f2);
        println!("p2 = ({}, {})", p2.f1, p2.f2);
        println!("p3 = ({}, {})", p3.f1, p3.f2);
    }
}
