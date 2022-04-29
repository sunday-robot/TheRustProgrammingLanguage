#![allow(unused)]
fn main() {
    if true {
        // IPアドレスの例
        let ip_address_kind = IpAddressKind::V4(1, 2, 3, 4);
        let ip_address_kind_2 = IpAddressKind::V6(String::from("::1"));

        println!("{:?}", ip_address_kind);
        println!("{:?}", ip_address_kind_2);

        print_ip_address(ip_address_kind);
        print_ip_address(ip_address_kind_2);
    }

    if false {
        // コマンドの例
        let quit_cmd = Command::Quit;
        let move_cmd = Command::Move { x: 1, y: 2 };
        let move_cmd_2 = Command::Move { x: 3, y: 4 };

        println!("{:?}", quit_cmd);
        println!("{:?}", move_cmd);
        println!("{:?}", move_cmd_2);
    }

    if false {
        let m = Command::Write(String::from("hello"));
        m.call();
    }
}

fn print_ip_address(ip_address: IpAddressKind) {
    println!("IPアドレス : {:?}", ip_address);
}

#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// JavaやC#で同様なことをするには、抽象クラスCommandと、これを継承した具象クラス群ということになると思われる。
// Quitはシングルトン、他の3クラスは通常のクラスということになる。
// ただし、上記の実装をしてしまうと、ユーザー側で勝手にCommandクラスのサブクラスを作れてしまうので、同じとは言えない。

// Commandにメソッドを追加できるとのことだが、
// チュートリアルにはそれしか記述されておらず、「だから何？」という感想しかない。
// チュートリアルは完成度が低いなあ…
// ↑enumとstructに似た点が多いということを示すのがチュートリアルの意図したことらしい。
impl Command {
    fn call(&self) {
        // メソッド本体はここに定義される
    }
}
