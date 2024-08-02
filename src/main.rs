fn main() {

    let _dont_do_this = IpAddrDontDoThis {
        ip_addr_kind: IpAddrKindDontDoThis::V4,
        address: String::from("192.168.11.0"),
    };

    let _ip_kind = IpAddrKind::V4(192,168,11,0);

    println!("Done!");
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
    ip_addr_kind : IpAddrKindDontDoThis,
    address: String
}