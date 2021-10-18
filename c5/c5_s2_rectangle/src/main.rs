#[derive(Debug)]
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
}

#[derive(Debug)]
struct TestMore {
    arg1: String,
    arg2: u32,
    arg3: (u32, String, f32),
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;

    // let rect1 = (30, 50);

    let test = TestMore {
        arg1: String::from("test"),
        arg2: 123,
        arg3: (100, String::from("hello"), 3.14),
    };

    println!("{:#?}", test);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect1 is: {:#?}", rect1);
    // println!("{}", rect1.0);

    let s = String::from("Hello");
    string_ownership(s);
}

// fn area(width: u32, height: u32) -> u32 {
    // width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
    // dimensions.0 * dimensions.1
// }

// fn area(rect: &Rectangle) -> u32 {
    // rect.height * rect.width
// }

// move ownership
fn string_ownership(s: String) {
    println!("{}", s);
}

