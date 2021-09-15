use std::io;
fn main() {
    loop {
        let mut degree = String::new();
        let mut convert_flag = String::new();
        println!("What would you want to convert?");
        println!("  1. C° --> F°");
        println!("  2. F° --> C°");
        println!("  0. exit");
        println!("Please input your choice:(tap enter to confirm)");

        io::stdin()
            .read_line(&mut convert_flag)
            .expect("Error in read_line");

        let convert_flag: u32 = convert_flag.trim().parse()
            .expect("Please input a valid number");

        if convert_flag == 1 {
            println!("Please input a Celsius degree: ");
            io::stdin()
                .read_line(&mut degree)
                .expect("Error read_line");
            let degree: f64 = degree.trim().parse()
                .expect("Please input a number");
            println!("{} C° is {} F°.", degree, convert_c2f(degree)); 
        } else if convert_flag == 2 {
            println!("Please input a Fahrenheit degree: ");
            io::stdin()
                .read_line(&mut degree)
                .expect("Error read_line");
            let degree: f64 = degree.trim().parse()
                .expect("Please input a number");
            println!("{} F° is {} FC.", degree, convert_f2c(degree)); 
        } else if convert_flag == 0 {
            break;
        } else {
            println!("Please input a valid choice!");
        }
    }
}

fn convert_f2c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn convert_c2f(c: f64) -> f64 {
    32.0 + c * 1.8
}
