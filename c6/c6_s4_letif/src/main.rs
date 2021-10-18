fn main() {
    let some_u8 = Some(3);
    match some_u8 {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8 {
        println!("three!");
    } else {
        println!("not three!");
    }
    let some_u8 = Some(4);
    if let Some(3) = some_u8 {
        println!("three!");
    } else {
        println!("not three!");
    }
}
