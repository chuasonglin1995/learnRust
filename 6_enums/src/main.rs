// instead of being tempted to put enums in structs
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// ===== Can put data directly into enum =====
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// This would be similar to a struct
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// same as above
/**
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
But
*/

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// defined in the standard library already
// enum Option<T> {
//     None,
//     Some(T),
// }


fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello"));
    let msg4 = Message::ChangeColor(255, 0, 0);

    match msg1 {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to red: {}, green: {}, blue: {}", r, g, b),
    }

    println!("Hello, world!");

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    //let absent_number: Option<i32> = None;

}


// Example of using option<T>
/**
#[derive(Debug)]
struct Transaction {
    id: u32,
    amount: f64,
    note: Option<String>,
}

impl Transaction {
    fn new(id: u32, amount: f64, note: Option<String>) -> Self {
        Transaction { id, amount, note }
    }

    fn process(&self) {
        println!("Processing transaction ID: {}", self.id);
        println!("Amount: ${}", self.amount);
        
        match &self.note {
            Some(note) => println!("Note: {}", note),
            None => println!("No note provided"),
        }
    }
}

fn main() {
    let transaction1 = Transaction::new(1, 100.0, Some(String::from("Payment for services")));
    let transaction2 = Transaction::new(2, 50.0, None);

    transaction1.process();
    transaction2.process();
}
    
*/