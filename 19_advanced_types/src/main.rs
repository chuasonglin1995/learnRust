



fn main() {
    type Kilometers = i32;

    // This is errorneous because we are mixing up Kilometers and i32. There is no type checking.
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // create new type is mainly to reduce code duplication and increase type safety
    type Trunk = Box<dyn Fn() + Send + 'static>;

    let f = Thunk::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    // instead of
    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    // ====== The NEVER type ======
    // for functions that either loop indefinitely or terminate the programe. 
    // indicates that a function will not return to the caller. 
    fn always_panics() -> ! {
        panic!("This function never returns!");
    }

    
   // ====== Dynamically Sized Types and the Sized Trait ======
   // by default it will always be treated as sized. T: Sized
   fn generic<T: ?Sized>(t: &T) {
        // --snip--
    }

}
