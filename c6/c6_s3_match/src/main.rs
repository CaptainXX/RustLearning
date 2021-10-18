enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let a_penny = value_in_cents(Coin::Penny);
    println!("One penny is {}", a_penny);

    let a_quarter = value_in_cents(
        Coin::Quarter(UsState::Alabama
        ));
    println!("One quarter is {}", a_quarter);

    let x = Some(5);
    let y = add_to_some(x);
    let z = add_to_some(y);


}

fn add_to_some(op: Option<i32>) -> Option<i32> {
    match op {
        None => None, // if None isn't processed, this code won't compile
        Some(i) => Some(i + 1),
    }
}