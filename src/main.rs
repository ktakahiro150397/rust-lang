fn main() {
    // println!("Hello, world!");

    // mutを付ける場合、そのインスタンス全体が可変になるぞ！
    // let mut user = User {
    //     username: String::from("John"),
    //     email: String::from("test@test.com"),
    //     age: 27,
    // };

    // user.username = String::from("Jane");

    let user = build_user("John".to_string(), "test@test.com".to_string(), 27);

    println!("User: {}", user.username);
    println!("User: {}", user.email);

    // 別のユーザーをインスタンスから生成
    // let user2 = User {
    //     username: "Jane".to_string(),
    //     email: user.email,
    //     age: user.age,
    // };

    // ..userでuserの残りのフィールドを使うことができる　便利すなあ
    let user2 = User {
        username: "Jane".to_string(),
        ..user
    };

    println!("User2: {}", user2.username);
    println!("User2: {}", user2.email);
}

fn build_user(username: String, email: String, age: u8) -> User {
    // フィールド名と同一の変数を使うと省略記法が使える dartの{this.param,...}みたいな感じ
    return User {
        username,
        email,
        age,
    };
}

struct User {
    username: String, // &strで文字列の参照だけを保持しておけばええやんと思うが、それはダメらしい。構造体自身がこのデータを所有する必要がある
    email: String,
    age: u8,
}
