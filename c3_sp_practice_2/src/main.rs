use std::io;

fn main() {
    loop {
        println!("Which fibonacci you want? (0 to exit)");
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Error read_line n");

        let number: u32 = number.trim().parse()
            .expect("Please input a valid interger!");
        
        if number == 0 {
            break;
        }

        println!("{}th fibonacci is {}", number, fibonacci(number));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    } else if n == 3 {
        return 2;
    }

    let mut x0: u32 = 1;
    let mut result: u32 = 2;
    for _ in 4..=n {
        let tmp = result;
        result = x0 + result;
        x0 = tmp;
    }

    result
}
