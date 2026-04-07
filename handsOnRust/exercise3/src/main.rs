struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, amount: u32) {
        self.width *= amount;
        self.height *= amount;
    }
}

fn main() {
    let mut rectangle = Rectangle {width: 10, height: 20};
    println!("{}", rectangle.area());

    rectangle.scale(2u32);
    println!("{}", rectangle.area());

    let mut temp_rectangle = Rectangle {width: 320, height: 3};
    println!("{}", rectangle.can_hold(&temp_rectangle));
    temp_rectangle = Rectangle {width: 2, height: 320};
    println!("{}", rectangle.can_hold(&temp_rectangle));
    temp_rectangle = Rectangle {width: 3, height: 3};
    println!("{}", rectangle.can_hold(&temp_rectangle));
}