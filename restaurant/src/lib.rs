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
}


pub fn eat_at_restaurant() {
    // Absolute path
    // crate でクレートルートからスタート
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // 現在のモジュールからスタート
    front_of_house::hosting::add_to_waitlist();

    // * front_of_houseはeat_at_restaurantと兄弟のモジュールなため、pubなしで参照できる
    // その子要素はpubが付与されていない場合は参照不可
}
