/// 制御構文について既存の言語と比較している

fn main() {
    c_like_for_study();
}

fn c_like_for_study() {
    // 0から9まで10回繰り返す。以下のCプログラム相当の処理。
    // for (int i = 0; i < 10; i++)
    // 　　printf("%d\n", i);

    // Cのforよりも、C#のforなどに近いもの。
    for i in 0..10 {
        println!("{}", i);
    }
}
