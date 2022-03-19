fn main() {
    let user = User {
        // 全メンバーの初期化が必要...
        username: String::from("akiyama"),
        email: String::from("sgx03150@nifty.com"),
        sign_in_count : 1,
        active: true,
    };

    println!("usermame = {}", user.username);

    // mutableなUserオブジェクト
    let mut second_user = User{
        username: String::from("sato"),
        email: String::from("sato@yahoo.co.jp"),
        sign_in_count : 2,
        active: false,
    };

    // mutableなオブジェクトは全メンバーが書き換え可能…
    second_user.username = String::from("suzuki");
    second_user.active = true;
    println!("usermame = {}", second_user.username);
    println!("active = {}", second_user.active);

    let third_user = build_user(String::from("sss@gmail.com"), String::from("yamada"));

    let forth_user = User {
        email:String::from("ttt@microsoft.com"),
        ..third_user    // 他の項目がthird_userと同じ場合、このように記述することで、username::third_user.username, ...などと書かなくてもよい。(だが、こんな機能使うことがあるのかな？)
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,   // このように記述するのが一応正規(?)なものだが、(次の行に続く)
        username,   // 構造体のフィールド名と、変数名が同じ場合は、このような記述ができる。
        active: true,
        sign_in_count: 1,
    }
}
