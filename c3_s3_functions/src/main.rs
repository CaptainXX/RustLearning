fn main() {
    println!("Hello, world!");

    another_function();
    show_a_number(100);
    rust_add(20, 30);

    let y = {
        let x = 5;
        x * 3
    };
    println!("y = {}", y);

    let foo_is = foo_return_23();
    println!("foo_return: {}", foo_is);

    let foo_is = foo_use_return();
    println!("foo_use_return: {}", foo_is);

    let x = plus_one(10);
    println!("10 puls one is {}", x);
}

fn another_function() {
    println!("Another function.")
}

fn show_a_number(x: i32) {
    println!("x is: {}", x);
}

fn rust_add(x: i32, y: i32) {
    println!("x + y = {}", x + y);
}

fn foo_return_23() -> i32 {
    23;
}

fn foo_use_return() -> i32 {
    let x = 2;
    let y = 4;
    return x | y;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
