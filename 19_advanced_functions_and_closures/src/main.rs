fn main() {
    // ======== function pointers ========
    // nothing much, just putting functions as arguments
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    // ======== returning closures ========
    // instead of `fn returns_closure() -> dyn Fn(i32) -> i32 {`
    // error because the size of the return type isn't known at compile time
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
