use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let word = &args[1];
    let mut vowels: Vec<&str> = Vec::new();
    vowels.push("a");
    vowels.push("e");
    vowels.push("i");
    vowels.push("o");
    vowels.push("u");
    println!("Word is: {}", word);
    let front = &word[0..1];
    let mut out_word = String::new();
    let mut is_moded = false;
    for v in vowels {
        if front == v {
            out_word = format!("{}-hay", &word[..]);
            is_moded = true;
            break;
        }
    }
    if !is_moded {
        out_word = format!("{}-{}ay", &word[1..], front)
    }
    println!("Word after processing: {}", out_word);

}
