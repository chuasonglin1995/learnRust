mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super goes back to the parent module, the root
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup, // enum variants are public by default
        Salad,
    }

    impl Breakfast {
        pub fn summer (toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change the bread again
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    use super::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        // this would error because shortcut does not apply wihtin customer module. need that use above
        hosting::add_to_waitlist(); 
    }
}

// ===== re-exporting names with pub use =====
// pub use crate::front_of_house::hosting;


// ===== external packages =====
// use std::cmp::Ordering;
// use std::io;
// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// try not to do this, hard to tell what names are in scope
// use std::collections::*;