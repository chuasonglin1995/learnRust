fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // rust can infer the type

    let mut v2 = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    
    // ===== if there is no reference to the vector =====
    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // this will cause panic
    let does_not_exist = v.get(100); // returns None without panicking

    // ===== if there is a reference to the vector =====
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // would error out later because reference here still in use
    // v.push(6); 
    // println!("The first element is: {first}");
    // this is an important concept because vector put values next to each other
    // pushing a new value might require allocating new memory and copying the old elements to the new space
    // then the reference would be pointing to deallocated memory

    // ===== Iterating over the values in a vector =====
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // to change the value that mutable reference refers to, we have to use * to get the value in i
        *i += 50;
    }

    // ===== Using an Enum to Store Multiple Types =====
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}