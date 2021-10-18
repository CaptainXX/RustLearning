fn main() {
    let a = 5;
    let b = a;
    println!("a = {}, b = {}", a, b);

    let s1 = String::from("Hello");
    let s2 = s1; // s1 is invalid now
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // this will cause error
}
