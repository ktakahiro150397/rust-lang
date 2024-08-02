mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // pubはモジュール名・関数名全てに付与して初めて公開できる

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        super::serve_order();
        cook_order();
    }

    fn cook_order() {}

    pub struct Breakfast{
        pub toast: String,
        seasonal_fluit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fluit: String::from("peaches"),
            }
        }
    }

    // 列挙体は型にpubを付与しておけば、値まるごと公開できる
    // 一部のみ非公開にすることはできない
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


pub fn eat_at_restaurant() {

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // 構造体を初期化する関連関数がないと、pubではないフィールドを初期化する手段はない
    // let meal_private = back_of_house::Breakfast {
    //     toast: String::from("wheat"),
    //     seasonal_fluit: String::from("blueberries"),
    // };

    // 季節のフルーツを知ることも、変更することもできない
    // meal.seasonal_fluit = String::from("blueberries");


    // Absolute path
    // crate でクレートルートからスタート
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 現在のモジュールからスタート
    front_of_house::hosting::add_to_waitlist();

    // * front_of_houseはeat_at_restaurantと兄弟のモジュールなため、pubなしで参照できる
    // その子要素はpubが付与されていない場合は参照不可
}
