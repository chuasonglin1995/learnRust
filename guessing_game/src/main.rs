// cargo doc --open // will build documentation provided by all dependencies locally

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            // & indeicate that the argument is a reference
            // references are also immutable by default. Hence the need to specify 'mut'
            .read_line(&mut guess)
            // Result's variants are Ok or Err
            // And if Err value, expect will cause the program to crash and display the message
            // if you dont call expect, problem will compile and you get a warning
            .expect("Failed to read line");
    
        // shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables
        // if parse is successful, it will return an Ok value that contains the number
        // if parse fails, it will return an Err value that contains more information about the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}