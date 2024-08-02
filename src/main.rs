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
}

fn build_user(username: String, email: String, age: u8) -> User {
    return User {
        username,
        email,
        age,
    };
}

struct User {
    username: String,
    email: String,
    age: u8,
}
