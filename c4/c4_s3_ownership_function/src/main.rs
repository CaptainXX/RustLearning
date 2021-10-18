fn main() {
    let s = String::from("Hello");

    takes_ownership(s);

    // println!("{}", s); // this cause error, s is invalid

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("World!");

    let s3 = takes_and_give_ownership(s2);

    println!("{}, {}", s1, s3);

    let (s1, len) = multi_return(s1);

    println!("The length of String \"{}\" is {}", s1, len);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let s = String::from("Hello");
    s
}

fn takes_and_give_ownership(a_string: String) -> String {
    a_string
}

fn multi_return(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
