use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};


fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        }
    };

    // ====== Using unwrap_or_else =====
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {error:?}");
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // });

    // ===== Using expect =====
    // Most programmers prefer to use expect instead of unwrap_or_else
    // let greeting_file = File::open("hello.txt")
    //   .expect("hello.txt should be included in this project");

    // ===== Propagating errors =====
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");
    
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
    
        let mut username = String::new();
    
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e), // dont need return here as this is the last expression
        }
    }

    fn read_username_from_file_2() -> Result<String, io::Error> {
        // using ? converts the error to the return type of the function
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    //  chain method calls
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // using read_to_string method
    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // If the value is None, the None will be returned early from the function at that point. 
    // If the value is Some, the value inside the Some is the resultant value of the expression. And function continues
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }


    // ===== Creating Custom types for validation =====
    // Can also do this to do validation checks so in 'new' function, we check the value type. 
    pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }
    
            Guess { value }
        }
    
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let guess = Guess::new(-11);

    // Cases where you want to panic
    // - When you have more information than the current context. 
    // - During testing, you can use unwrap_or_else to provide more information about the panic
    // - if you are a package author, you may want to use panic to alert the user of your library to a problem in their code
    
}