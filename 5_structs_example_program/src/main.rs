fn areaTuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn areaStruct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

# [derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectange is {} square pixels.",
        areaStruct(&rect1)
    );

    println!("rect1 is {rect1:?}");
    println!("rect1 is {rect1:#?}");
    dbg!(&rect1); // macro prints to the standard error console stream (stderr) as opposed to stdout

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);


}

