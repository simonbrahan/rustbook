use front_of_house::hosting;
use back_of_house::{Appetiser, Breakfast};

mod back_of_house {
    #[derive(Debug)]
    pub enum Appetiser {
        Soup,
        Salad
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    back_of_house::fix_incorrect_order();

    let meal = Breakfast::summer("rye");

    println!("{} toast please", meal.toast);
    dbg!(Appetiser::Soup);
    dbg!(Appetiser::Salad);
}
