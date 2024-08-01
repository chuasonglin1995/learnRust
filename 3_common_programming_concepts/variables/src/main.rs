fn main() {
    // ===== mutables =====
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // ===== constants =====
    // Constants are always immutable
    // Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of your code need to know about.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // ===== shadowing =====
    {   
        // this creates a new variable x in the inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); //12
    }

    println!("The value of x is: {x}");

    // in this case, the first spaces is a string, 
    // and the second spaces is a number
    let spaces = "   ";
    let spaces = spaces.len();

}