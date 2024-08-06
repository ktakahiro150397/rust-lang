fn main() {
    // i32を格納するベクタを作成
    let v: Vec<i32> = Vec::new();

    // vec!は要素を格納するベクタを新しく作成するマクロ
    let v = vec![1,2,3];

    // 要素の追加
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2]; // 参照を得る
    println!("The third element is {}", third);

    // getメソッドでもよい Option<T>で返ってくる
    match v.get(2) {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element."),
    }
    match v.get(99) {
        Some(value) => println!("The 100th element is {}", value),
        None => println!("There is no 100th element."),
    }

    // 不変な参照と可変な参照は同時に存在できない。これはベクタの要素を通して全体に適用される。
    let mut v = vec![1,2,3,4,5]; // 可変
    let first = &v[0]; // 最初の要素への不変な参照

    v.push(6); // ベクタ末尾に追加

    println!("Vector is : {:?}", v);

    // 最初の要素を参照するとエラー
    // ベクタは全体が別の領域にコピーされる可能性があるため、コンパイルチェックが入る
    // println!("First element is : {}", first);

    // ベクタの不変参照を列挙できる
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        // 列挙中のコレクションを変更することができるんすね
        *i += 50; // 参照外し演算子*：後ほど出てくる
    }
    println!("Vector is : {:?}", v);


    // enumをベクタに格納できる
    // enumはさらに値を持てるため、異なる型の要素をベクタに持つことができる
    let cells = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
        ];
    println!("{:?}", cells);

    for cell in &cells {
        // matchで列挙型の値を取り出せる
        match cell {
            SpreadSheetCell::Int(value) => println!("Int value is {}", value),
            SpreadSheetCell::Text(value) => println!("Text value is {}", value),
            SpreadSheetCell::Float(value) => println!("Float value is {}", value),
            _ => println!("Other value"),
        }
    }

    // 文字列について
    // Vec<T>とほぼ共通 -> 文字のコレクションであるため
    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // String初期化はto_string()かString::from()を使う
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    println!("s:{}", s);

    // 文字列スライスを追加できる
    let mut s = "foo".to_string();
    s.push_str("bar");

    println!("s:{}", s);

    let mut s = "lo".to_string();
    s.push('l'); // 1文字だけ追加できる シングルクォートを使ってcharリテラルにする

    println!("s:{}", s);
    println!("s:{}", &s[..]);

    // 文字列インデックスアクセスは禁止
    // UTF-8のため、有効な文字かバイトかが不明
    // 検証する必要があるため、O(1)にならないため
    // println!("s[0]:{}", s[0]);
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Text(String),
    Float(f64),
}