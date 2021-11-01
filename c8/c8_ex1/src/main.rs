use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let filename = String::from("test.txt");
    let err_msg = format!("Something wrong when reading file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect(&err_msg[..]);
    let mut numbers: Vec<i32> = Vec::new();
    for line in contents.split_terminator("\n") {
        numbers.push(FromStr::from_str(line).unwrap());
    }
    println!("{:?}", numbers);
    
    let mut sum: f64 = 0.0;
    let mut median: i32 = 0;
    let mut mode: HashMap<i32, i32> = HashMap::new();
    let n_numbers = numbers.len();
    numbers.sort();
    median = numbers[n_numbers / 2];
    for number in numbers {
        let count = mode.entry(number).or_insert(0);
        *count += 1;
        sum += number as f64;
    }
    let mean = sum / (n_numbers as f64);
    println!("median is {}, mean is {}", median, mean);
}
