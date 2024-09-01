
enum ListOld {
    Cons(i32, List), // recursive type
    Nil,
}

// when you write it like that, rust knows that it only needs to find 
// enough space for a pointer to a List, not the List itself
// Becaue box is a smart pointer
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
 let b = Box::new(5);
 println!("b = {b}");
 
 let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
 // Print the list
 print_list(&list);
}

fn print_list(list: &List) {
    match list {
        List::Cons(value, next) => {
            print!("{} ", value);
            print_list(next);
        }
        List::Nil => {
            println!();
        }
    }
}