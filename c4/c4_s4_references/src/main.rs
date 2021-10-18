fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of \"{}\" is {}", s1, len);

    change_borrowing(&mut s1);
    println!("s1 after change: {}", s1);

    let mut s2 = String::from("captain");
    {
        let ref2 = &mut s2;
        println!("ref2 {}", ref2);
    }
    let immut_ref = &s2;
    println!("immut_ref {}", immut_ref);

    let ref1 = &mut s2;
    println!("ref1 {}", ref1);

    let mut_ref2 = &mut s2;
    println!("mut_ref2 {}", mut_ref2);

    println!("{}", dangle());
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change_borrowing(s: &mut String) {
    s.push_str(", world!");
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}
