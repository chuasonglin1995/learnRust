/**
 * 
 * ===== match ARMS ======
 *  match x {
 *   None => None,
 *   Some(i) => Some(i + 1),
 * }
 * 
 * ==== conditional if let Expressions =====
 * if let Some(color) = favorite_color {
 * } else if is_tuesday {
 * } else if let Ok(age) = age {
 * } else {
 * }
 * 
 * ===== while let Conditional Loops =====
 * let mut stack = Vec::new();
 * stack.push(1);
 * stack.push(2);
 * stack.push(3);
 * while let Some(top) = stack.pop() {
 *   println!("{top}");
 * }
 * 
 * ===== for Loops =====
 * let v = vec!['a', 'b', 'c'];
 * for (index, value) in v.iter().enumerate() {
 *    println!("{index}: {value}");
 * }
 * 
 * ===== let Statements =====
 * let (x, y, z) = (1, 2, 3);
 * 
 * ===== Function Parameters =====
 * fn print_coordinates(&(x, y): &(i32, i32)) {
 *   println!("Current location: ({x}, {y})");
 * }
 * 
 * fn main() {
 *   let point = (3, 5);
 *   print_coordinates(&point);
 * }
 * 
 * ====== Refutability: Whether a Pattern Might Fail to Match =====
 * Refutable: Patterns that might not match for some possible value
 *   - if let Some(x) = some_option_value -> if it doesn not match, it will not run that code block
 *   - match some_value {
 * Irrefutable: Patterns that will match for any possible value passed.
 *   - let x = 5;
 *   - let statements
 *   - for loops
 *  
 * 
 */


 /**
  * Pattern Syntax


    ===== Multiple Patterns =====
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    ===== Matching Ranges of Values with ... =====
    let x = 5;
    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("anything"),
    }

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    ===== Destructuring Structs =====
    let Point { x: a, y: b } = p;

    // can also match just one of them
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    ===== Destructuring Enums =====
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}")
        }
    }

    // ===== Destructuring Nested Structs and Enums =====
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // ===== Destruction Structs and Tuples =====
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ===== Ignoring Parts of a Value with a Nested _ =====
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    // ===== ignoring an used variable with _ =====
    let _x = 5;


    // ===== Ignoring Remaining Parts of a Value with .. =====
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // ===== Extra Conditionals with Match Guards =====
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }


    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // This shadows the outer `y` --> bad because now the outer `y` cannot be accessed
        _ => println!("Default case, x = {x:?}"),
    }


    match x {
        4 | 5 | 6 if y => println!("yes"), => is more like (4 | 5 | 6) if y
        _ => println!("no"),
    } 

    // ===== @ Bindings =====
    // Using when you need to 
    //  - capture a value for use in the match arm
    //  - test that value to see whether it matches a pattern
    //  - apply addtonal condition or destructuring the value further
    let x = Some((3, 4));

    match x {
        Some((a @ 1..=5, b @ 1..=5)) => println!("Matched, a = {a}, b = {b}"), // `a` and `b` are bound to the values inside the tuple if they are between 1 and 5
        Some((a, b)) => println!("Matched, a = {a}, b = {b}"), // `a` and `b` are bound to the values inside the tuple for any other values
        None => println!("No value"),
    }

    let order = Order {
        id: 123,
        status: OrderStatus::Shipped,
    };

    match order {
        // You can match directly on the enum variants without using @ bindings if you don't need to capture the value for additional conditions or destructuring.
        // if its part of the pattern matching, then you cannot access the value in the exeuction arm!
        Order { id, status: s @ OrderStatus::Pending } => {
            println!("Order {id} is pending. Status: {:?}", s);
        }
        Order { id, status: s @ OrderStatus::Shipped } => {
            println!("Order {id} has been shipped. Status: {:?}", s);
        }
        Order { id, status: s @ OrderStatus::Delivered } => {
            println!("Order {id} has been delivered. Status: {:?}", s);
        }
    }

  */


fn main() {
    println!("Hello, world!");
}
