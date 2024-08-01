// Stack vs Heap
// Stack is like a queue - last in, first out. Size cannot be changed.
// Heap is less organized, you need to tell it how many space required, and it returns you with the location
// Heap is slower then stack because allocating space on heap requires more work,
//   allocator must first find big enough space to hold the data and then perform bookkeeping to prepare for next alllocation

fn main() {
     // string literals, immutable
    let s = "hello";
    println!("{s}");
    
    // data allocated on the heap, store an amount of text that is unknown to us at compile time
    // `String::from` is to request for memory it needs, universal in programming languages
    // it is our responsibility to recognize when memory is no longer being used and to call code to explicitly free it
    let f = String::from("hello world"); 
    println!("{f}");

    let s1 = String::from("hello");
    let s2 = s1; //** To prevent double free error, rust will consider s1 no long valid at this point

    // println!("{s1}, world!"); Will error out

    // ===== Variables and Data Interacting with Clone ======
    let s1 = String::from("hello");
    let s2 = s1.clone(); //copies the heap data instead of the `metadata` like pointer idx, length, capacity
    // clone() is expensive
    println!("s1 = {s1}, s2 = {s2}");

    // ===== Stack-Only Data: Copy ======
    // this is okay becaue integershave a konwn size at compile timeand stored entirely on the stack
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    // ===== Ownership and Functions ======
    // Passing a value to a function will move or copy and might cause it to go out of scope
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    // ====== References and Borwwong =====
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut s = String::from("hello");
     change(&mut s); 

}  // at this point, `f` goes out of scope, and rust calls a special function for us (drop)


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}