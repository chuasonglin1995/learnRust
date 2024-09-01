/**
 * Explain deref:
 * Let's imagine you have a special toy box. Inside this toy box, you have a toy car. 
 * Normally, to play with the toy car, you need to open the toy box and take the car out. 
 * But what if the toy box was magical and you could play with the toy car without opening the box?
 * In Rust, Deref is like that magic. It lets you use the toy car directly, even though it's inside the toy box. 
 * So, you don't have to worry about opening the box every time you want to play with the car.
 */

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


// the deref method gives the compiler the ability to take a value
// of any type to get a &reference to that value
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // returns first value in a tuple struct
    }
}


fn hello(name: &str) {
    println!("Hello, {name}!");
}
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
