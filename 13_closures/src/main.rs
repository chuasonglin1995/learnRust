/**
 * closures are defined using the | syntax for parameters and the body of the closure in the curly braces.
 * let closure = |param1, param2| {
 *  // Closure body
 *   param1 + param2
 * };
 */

// Closures are like functions, but they will infer the types of the parameters and return value
// parameters passed in do need to have consistent types (cannot be one time string, another time integer)
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v4 = |x|               x + 1  ;

use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // The unwrap_or_else method on Option<T> takes one argument: a closure without any arguments 
    // that returns a value T (the same type stored in the Some variant of the Option<T>, in this case ShirtColor).
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {

    // ===== Capturing the Environment with Closures =====
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red, 
            ShirtColor::Blue
        ],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );


    //======== Capturing references or Moving Ownership =========
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // move keyword to take ownership of the environment
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // ====== Moving Captured Values Out of Closures and the Fn Traits ======
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // sort_by_key method is defined to take an FnMut closure because it calls the closure multiple times, once for each item
    // |r| r.width does not capture, mutate, or move out anything from its envrionment
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    // The below will not work
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");

    // // this is trying to move captured value out of the closure
    // // it transfers ownership of value to the sort_operations vector
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");


}
