
// ===== Below code are the same ======
// using match vs if let
// if you need it to be more verbose, use if let
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1,
}

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
// --------------

fn main() {
    println!("Hello, world!");
}
