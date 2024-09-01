// In rust, you can specify that a particular bit of code be run whenever a value
// goes out of scope. 'drop'

struct CustomSmartPointer {
    data: String,
}

// oh so these codes are called when CustomSmartPointer goes out of scope
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}