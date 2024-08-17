// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Three rules the compiler uses to figure out lifetime parameter
// 1. In a function, one param, gets one lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime params
// 3. If there are multiple input lifetime params, and one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime params

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ===================
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // =======================================
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // This would error out, string2 does not live long enough
    // println!("The longest string is {result}");

    // ======= Lifetime Annotations in Struct Definitions ============
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // ====== Static lifetime ======
    let s: &'static str = "I have a static lifetime.";
    // always available
    // Warning: do not do this just so that you can resolve a dangling reference, mismatch of avilable lifetimes. 
}


