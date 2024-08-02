fn main() {
    let _dont_do_this = IpAddrDontDoThis {
        ip_addr_kind: IpAddrKindDontDoThis::V4,
        address: String::from("192.168.11.0"),
    };

    let _ip_kind = IpAddrKind::V4(192, 168, 11, 0);

    let m = Message::Write(String::from("hello, enum!"));
    m.call();

    // Option<T> : 有効な値か、無効な値を表すenum
    let suspect_number : Option<i32> = None;
    let suspect_number2 : Option<i32> = Some(999); // Some(T_value)のような形式で設定する必要がある
    // let suspect_number3 : i32 = None; // i32にOption<T>::Noneは当然代入できない

    print_number(suspect_number);
    print_number(suspect_number2);

    // Option<i32>とi32を足すことはできず、コンパイルエラー
    // i32は必ず有効な値であることがコンパイラで保証されている！いいね
    // let sum = suspect_number + 30;

    let suspect_number4 = Some(100);

    print_number(suspect_number4);
    let suspect_number5 = add_number(suspect_number4, 30);
    print_number(suspect_number5);
   

    println!("Done!");
}

fn print_number(suspect_number:Option<i32>) {
match suspect_number {
        Some(value) => {
            println!("Value is valid! : {}", value);
        },
        None => {
            // ここでi32にアクセスすることはできない！よい思想ですね
            println!("Value is None!");
        }
    }
}

fn add_number(suspect_number:Option<i32>,add_number:i32) -> Option<i32> {
    // match suspect_number {
    //     Some(value) => {
    //         return Some(value + add_number);
    //     },
    //     None => None,
    // }

    // 1行で書くこともできます
    match suspect_number {
        Some(value) => Some(value + add_number),
        None => None,
    }
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
