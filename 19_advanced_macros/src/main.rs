// Difference between macros and function
//  - Macros are expanded before the compiler interprets the meaning of the code
//  - Downside of marcos is that you are writing rust code that writes rust code. So its alot more complex

// Simplified definition of vec! macro
// we wont be able to use a function to do the same because we wont know the numner or types of arguments up front
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}

