// rust uses snake case
// in rust you cannot do let x = y = 6

fn main() {
    println!("Hello, world!");
    another_function(32, 'h');

    // this is an expression that evaluates to 4
    // expressions do not end with semicolon
    let y = {
        let x = 3;
        x+1 // no semicolon
    };
    println!("{y}");

    let f = five();
    println!("{f}");

    let plus = plus_one(5);
    println!("{plus}");
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is {x}{unit_label}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}