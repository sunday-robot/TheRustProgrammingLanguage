#[allow(dead_code)]
fn main_1() {
    let r: &i32;
    {
        let x = 5;
        r = &x;
        println!("r = {}", r);
    }
    // rにはxの参照がセットされたが、この行はxのスコープ外である。
    // このため、以下の様にrの値を参照する実装はコンパイルエラーとなる。
    // println!("r = {}", r);
}

