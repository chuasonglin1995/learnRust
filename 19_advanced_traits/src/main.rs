
// ====== Trait with associated types ======
trait Container {
    type Item;  // associated type of item, can be i32, Vec<T>, whatever

    fn add(&mut self, item: Self::Item);
    fn removee(&mut self) -> Option<Self::Item>;
}

// Implement the trait for a Vec
impl Container for Vec<i32> {
    type Item = i32;

    fn add(&mut self, item: i32) {
        self.push(item);
    }

    fn removee(&mut self) -> Option<i32> { // use a diff name because remove is a method of Vec
        self.pop()
    }
}

// Implement the trait for a custom struct
struct MyContainer<T> {
    items: Vec<T>,
}

impl<T> Container for MyContainer<T> {
    type Item = T;

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn removee(&mut self) -> Option<T> {
        self.items.pop()
    }
}

// ======= Operator Overloading =======
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// You overwrite the + operator for Point
// doing overloading is mainly for benefits of type safety and encapsulation
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() { 
    let mut vec_container: Vec<i32> = Vec::new();
    vec_container.add(1);
    vec_container.add(2);
    println!("removeed from vec_container: {:?}", vec_container.removee());

    let mut my_container: MyContainer<i32> = MyContainer { items: Vec::new() };
    my_container.add(3);
    my_container.add(4);
    println!("removeed from my_container: {:?}", my_container.removee());

    // ======= Operator Overloading =======
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}