use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to readline.");

    let number: u32 = number
        .trim()
        .parse()
        .expect("Cannot parse to number!");

    if number < 5 {
        println!("{} < 5!", number);
    } else {
        println!("{} > 5!", number);
    }

    if number != 0 {
        println!("number is not 0");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let x = if number > 5 { 
        "More Than Five" 
    } else { 
        "Less Than Five" 
    };
    println!("x is {}", x);
}
