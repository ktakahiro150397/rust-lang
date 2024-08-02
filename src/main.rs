#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// impl <struct> で構造体にメソッドを追加できる
impl Rectangle {
    fn area(&self) -> u32 {
        // &self は構造体自身のインスタンスを指している
        self.width * self.height
    }

    fn setWidth(&mut self,width:u32){
        //可変でselfを受けることもできる。この場合、構造体インスタンス自体がmutである必要がある
        self.width = width;
    }

    fn square(size: u32) -> Rectangle {
        // インスタンスではなく、構造体に属する関連関数
        // <struct>::<function>(args) で呼び出す
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectanble is {} square pixels.",
        rect2.area()
    );
    println!("rect is {:#?}!", rect2);


    rect2.setWidth(40);
    println!(
        "The area of the rectanble is {} square pixels.",
        rect2.area()
    );
    println!("rect is {:#?}!", rect2);

    let square = Rectangle::square(3);
    println!(
        "The area of the rectanble is {} square pixels.",
        square.area()
    );

}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     // タプルで扱う場合
//     dimensions.0 * dimensions.1
// }

