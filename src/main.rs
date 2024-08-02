fn main() {
    let _dont_do_this = IpAddrDontDoThis {
        ip_addr_kind: IpAddrKindDontDoThis::V4,
        address: String::from("192.168.11.0"),
    };

    let _ip_kind = IpAddrKind::V4(192, 168, 11, 0);

    let m = Message::Write(String::from("hello, enum!"));
    m.call();

    println!("Done!");
}

#[derive(Debug)]
enum Message {
    // Structでも定義できるが、Message型を引数にとる関数を定義できる点でアドバンテージがある
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // enumのメソッドを定義できる
        // このメソッドは、Message型のインスタンスに対して呼び出すことができる

        // match式で、enumの値を確認し、その値を処理できる
        match self {
            Message::Write(arg) => {
                println!("Write: {}", arg);
            },
            // 分岐する際の引数はコンパイルチェックあり
            // Message::ChangeColor(arg) => {
            //     println!("ChangeColor!");
            // },
            _ => {
                println!("Other");
            }
        }
    }
}

enum IpAddrKind {
    // enumに、関連するデータ型を埋め込むことがｄけいる！
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddrKindDontDoThis {
    V4,
    V6,
}

struct IpAddrDontDoThis {
    // 構造体にenumと、それに紐づくデータ型を持ったこういう構造体はRustでは作らなくていい
    ip_addr_kind: IpAddrKindDontDoThis,
    address: String,
}
