fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    for i in v {
        println!("{}", i);
    }

    let mut v1 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v1[2];
    println!("The third element is {}", third);
    let x: Option<i32>;
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let third: &i32 = &v1[2];
    // third = &1;

    // v1.push(1);

    println!("Third is {} now", third);

    for i in &v1 {
        println!("{} in v1", i);
    }

    for i in &mut v1 {
        *i += 10;
        println!("Now {} in v1", i);
    }

    #[derive(Debug)]
    enum MulTypesInVec {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        MulTypesInVec::Int(10),
        MulTypesInVec::Int(2),
        MulTypesInVec::Float(20.12),
        MulTypesInVec::Text(String::from("Hello")),
    ];
    for i in &row {
        println!("{:?} in row", i);
    }
}
