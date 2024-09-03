// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}


// ======== Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T> ========
#[derive(Debug)]
enum ListMultiOwner {
    Cons(Rc<RefCell<i32>>, Rc<ListMultiOwner>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {a:?}");
    // println!("b after = {b:?}");
    // println!("c after = {c:?}");

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());


    // ======== Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T> ========

    let value = Rc::new(RefCell::new(5));

    let d = Rc::new(ListMultiOwner::Cons(Rc::clone(&value), Rc::new(ListMultiOwner::Nil)));

    let e = Rc::new(ListMultiOwner::Cons(Rc::new(RefCell::new(3)), Rc::clone(&d)));
    let f = Rc::new(ListMultiOwner::Cons(Rc::new(RefCell::new(4)), Rc::clone(&d)));

    *value.borrow_mut() += 10;

    // shows that all have the modifed value of 15
    println!("d after = {d:?}");
    println!("e after = {e:?}");
    println!("f after = {f:?}");
    // this technique is useful so that we have an outwardly immutable List value
    // but we can use the methods on RefCell<T> that provide interior mutability to modify the value as we need
}