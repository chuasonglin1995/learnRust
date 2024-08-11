use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // set 0 is no entry

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    // ===== Ownership =====
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // hashmap takes ownership of field_name and field_value
    // field_name and field_value are invalid at this point

    // ===== Adding a Key and Value only if the Key is not present =====
    scores.entry(String::from("Yellow")).or_insert(50);

    // ====- Updating a Value Based on the Old Value =====
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
