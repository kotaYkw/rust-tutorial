fn serve_order() {}

mod back_of_house{
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

    pub enum Appetizer {
        Soup,
        Sald,
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod front_of_house;

pub use crate::front_of_house::hosting; // Absolute path, re-exporting
use self::front_of_house::hosting; // Relative path
use std::collections::HashMap;
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
use std::io::{self, Write};
use std::collections::*;

fn function1() -> FmtResult {
    ok(())
}

fn function2() -> IoResult<()> {
    ok(())
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberry");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Sald;

    hosting::add_to_waitlist();
    let mut map = HashMap::new();
    map.insert(1, 2);


}