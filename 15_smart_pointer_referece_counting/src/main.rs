// Rust type Rc<T> is to enable multiple ownership of data; 
// Rc<T> is a "reference counting" smart pointer.
// We use Rc<T> type when we want to allocate memory on the heap
//   for multiple parts of the program to read and we can't determine
//   at compile time which part will finish using the data last.
// *Note: Rc<T> is only for use in single-threaded scenarios.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    } // c goes out of scope
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 4
}