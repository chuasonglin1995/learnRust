// string is actually a wrapper around Vec<u8> (a vector of bytes)

fn main() {
    // ===== Creating a New String =====
    // method 1
    let data = "initial contents"; // string literal
    let s = data.to_string();
    let s = "initial contents".to_string(); //  works on a literal directly:

    // method 2
    let s = String::from("initial contents");

    // ===== Updating a string =====
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str() appends a string slice to a String
    println!("{}", s); // prints foobar

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3); // prints Hello, world!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s4); // prints tic-tac-toe
    let s1 = String::from("tic");
    let s5 = format!("{s1}-{s2}-{s3}"); // takes references so this call doesnt take ownership of its of its parameters

    // ===== Indexing Into Strings =====
    let hello = "Здравствуйте";
    // let answer = &hello[0]; 
    // the '3' actually takes 2 bytes of storage. 
    // to avoid misunderstanding/ misuse, Rust refuse to compile these codes at all.
    // println!("{}", answer);

    let s = &hello[0..4];
    println!("{}", s); // prints Зд

    // ===== Methods for Iterating Over Strings =====
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
    
}
