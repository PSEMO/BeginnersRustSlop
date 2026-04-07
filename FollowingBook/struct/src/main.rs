//struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let number = 2;

    let rect = Rectangle {
        width: dbg!(30 * number),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    println!("Rectangle is {rect:?}");
    dbg!(&rect);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

//normal
//fn main() {
//    let width1 = 30;
//    let height1 = 50;
//
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(width1, height1)
//    );
//}
//
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

//tuple
//fn main() {
//    let rect = (30, 50);
//
//    println!(
//        "The area of the rectangle is {} square pixels.",
//        area(rect)
//    );
//}
//
//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}