fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    // ===== Signed vs Unsigned =====
    // signed and unsigned refer to whether it's possible for the number to be negative
    // when its safe to assume number is positive, use unsigned
    // i8 can store numbers from -128 to 127
    // u8 can store numbers from 0 to 255

    // ===== Interger Overflow =====
    // when compiling in debug mode, Rust will check for integer overflow and panic if it occurs
    // However, in release mode, Rust will not check for integer overflow. eg. u8 if assigned 256 wil become 0, 257 will become 1

    // ===== Floating-Point Types =====
    // Default is f64, becuase modern CPUs, its roughly the same speed as f32, but capabe of more precision
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // ===== Numeric Operations =====
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated = -5 / 3; // Results in -1 because the result of dividing by integer will always be integer
    println!("The value of truncated is: {truncated}");

    // remainder
    let remainder = 43 % 5; // result = 3
    println!("The value of remainder is: {remainder}");
    


}
