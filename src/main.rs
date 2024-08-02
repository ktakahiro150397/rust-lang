#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectanble is {} square pixels.",
        area(&rect2)
    );

    println!("rect is {:#?}!", rect2);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     // タプルで扱う場合
//     dimensions.0 * dimensions.1
// }

fn area(rect: &Rectangle) -> u32 {
    // 構造体で受けることで意味が明確になる
    // ただし、参照を使わないとムーブが発生する。基本的には参照を受けるようにしたほうがよさげ
    rect.width * rect.height
}
