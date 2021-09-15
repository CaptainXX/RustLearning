fn main() {
    // An example of float parse
    let test_float: f32 = "3.1415926".parse()
        .expect("Not a number!");
    
    println!("test_float is: {}", test_float);

    // Warpping
    let x: u32 = 255;
    let x = x + 1;
    println!("x is: {}", x);

    // float point
    let xf = 10.0 / 3.0;
    let yf: f32 = 3.322 + 2.0;
    println!("xf = {}, yf = {}", xf, yf);

    // Boolean
    let t = true;
    let k: bool = false;

    // Tuple
    let tup = (500, 5.3, 20);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    
    let x = tup.0;
    let z = tup.2;
    println!("x = {}, z = {}", x, z);

    // Array
    let months = ["January", "February", "March", "April"];
    let a: [u32; 3] = [2, 4, 6];
    let b = [10; 5];
    println!("The first month is: {}", months[0]);
    println!("a = [{}, {}, {}]", a[0], a[1], a[2]);
    println!("b = [{}, {}, {}, {}, {}]", 
        b[0], b[1], b[2], b[3], b[4]);
    use std::io;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
