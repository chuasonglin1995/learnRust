// qn: what is the difference between &string and &str?
fn main() {
    println!("Hello, world!");
    let s = "hello word";
    let word = first_word(s); // qn: can be s or &s?
    println!("{word}");

    // s.clear(); this would error out because its referenced later on

    let _hello = &s[0..5];
    let _world = &s[6..10];
    let _slice = &s[..2];
    let _slice = &s[3..];
    println!("{_slice}");

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn second_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     let wordCount = 1;
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
// }
