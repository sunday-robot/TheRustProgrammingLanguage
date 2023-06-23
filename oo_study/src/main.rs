use std::{f64::consts::PI, string};

/// Javaでいえば抽象基底クラスShape
/// Rustではメンバ変数はstruct、メソッドはtrait
/// Shapeに関してはメンバ変数はないのでtraitのみ
trait ShapeTrait: std::fmt::Debug {
    fn get_langth(&self) -> f64;
    fn get_area(&self) -> f64;
}

// Javaでいえば抽象基底クラスShapeを継承した同じく抽象基底クラスNamedShape
#[derive(Debug)]
struct NamedShape {
    pub name: String,
}
trait NamedShapeTrait: ShapeTrait {
    fn get_name(&self) -> &String;
}

// NamedShapeは抽象クラスでget_name()以外は実装しないのだが、そのようなことはRustではできないらしい。
// 「回避方法」として、デフォルト実装をするということらしいが、これでは全然ダメ。
// JavaやC#なら(多分C++でも)具象クラスで実装し忘れたらコンパイルエラーとなるのに、
// Rustではコンパイルエラーにならず、実行時にようやく実装忘れに気付くことができる。
// Rustの方針としてこれは全くダメなのでは。
impl ShapeTrait for NamedShape {
    fn get_langth(&self) -> f64 {
        panic!("need to be implemented");
    }
    fn get_area(&self) -> f64 {
        panic!("need to be implemented");
    }
}

impl NamedShapeTrait for NamedShape {
    fn get_name(&self) -> &String {
        &self.name
    }
}

struct NamedSquare {
    pub length: f64,
}

#[derive(Debug)]
struct Square {
    pub length: f64,
    pub name: String,
}
impl ShapeTrait for Square {
    fn get_langth(&self) -> f64 {
        self.length
    }
    fn get_area(&self) -> f64 {
        self.length * self.length
    }
}

impl NamedShapeTrait for Square {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Square {
    pub fn new(length: f64, name: String) -> Square {
        Square { length, name }
    }
}

#[derive(Debug)]
struct Circle {
    pub diameter: f64,
    pub name: String,
}

impl ShapeTrait for Circle {
    fn get_langth(&self) -> f64 {
        self.diameter
    }
    fn get_area(&self) -> f64 {
        PI * self.diameter * self.diameter / 4.0
    }
}

impl NamedShapeTrait for Circle {
    fn get_name(&self) -> &String {
        &self.name
    }
}

impl Circle {
    pub fn new(diameter: f64, name: String) -> Circle {
        Circle { diameter, name }
    }
}

fn print_shape(shape: Box<dyn ShapeTrait>) {
    print!(
        "length = {:?}, area = {:?}\n",
        shape.get_langth(),
        shape.get_area()
    );
}
fn print_named_shape(shape: Box<dyn NamedShapeTrait>) {
    print!(
        "name = {:?}, length = {:?}, area = {:?}\n",
        shape.get_name(),
        shape.get_langth(),
        shape.get_area()
    );
}

fn main() {
    let shapes: Vec<Box<dyn ShapeTrait>> = vec![
        Box::new(Square::new(1.2, String::from("square1"))),
        Box::new(Circle::new(2.0, String::from("circle2"))),
    ];

    for shape in shapes {
        print_shape(shape);
    }

    let named_shapes: Vec<Box<dyn NamedShapeTrait>> = vec![
        Box::new(Square::new(1.2, String::from("square1"))),
        Box::new(Circle::new(2.0, String::from("circle2"))),
    ];
    for shape in named_shapes {
        // let a : ShapeTrait = shape.into();
        print_shape(shape);
        print_named_shape(shape);
    }
}
