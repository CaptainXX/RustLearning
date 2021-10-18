fn main() {
    // An example of variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // An example of const
    const MAX_POINTS: u32 = 1000_000;
    println!("The const MAX_POINTS is: {}", MAX_POINTS);

    // An example of shadowing
    let y = 3;
    let y = y + 1;
    let y = y * 3;
    println!("Final y is: {}", y);

    // Another example of shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces is: {}", spaces);
}
