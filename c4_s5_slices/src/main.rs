fn main() {
    let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{}, {}", hello, world);

    let word = first_word(&s);
    println!("{}", word);

    let word = second_word(&s);
    println!("{}", word);

    
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if first_index == 0 && item == b' ' {
            first_index = i + 1;
        } else if item == b' ' {
            return &s[first_index..i];
        }
    }

    if first_index != 0 {
        return &s[first_index..];
    } else {
        return ""
    }
}
