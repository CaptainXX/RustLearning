fn main() {
    let mut s = String::new();
    let data = "Hello string";
    s = data.to_string();
    println!("{}", s);

    s = "Hello again".to_string();
    println!("{}", s);

    s = String::from("A string");
    println!("{}", s);

    s = String::from("Rust 牛逼");
    println!("{}", s);

    s.push_str(", 阿巴阿巴");
    println!("{}", s);

    let mut front = String::from("Hello");
    let end = " End";
    front.push_str(end);
    println!("Now, front is: {}, end is: {}", front, end);

    s.push('。');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("World.");
    let s3 = s1 + &s2;

    println!("{}", s3);
    println!("s2 is valid?: {}", s2);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let s = format!("{}-{}-{}", t1, t2, t3);
    println!("{}", s);

    let s1 = String::from("哈哈哈");
    let ss = &s1[0..3];
    println!("{}", ss);
    for c in s1.chars() {
        println!("{}", c);
    }
    for b in s1.bytes() {
        println!("{}", b);
    }

    let s1 = String::from("hello");
    let ss = &s1[1..2];
    println!("{}", ss);
    for c in s1.chars() {
        println!("{}", c)
    }

    let hello = String::from("Hello");
    for mut c in hello.chars() {
        c = 'w';
    }
    println!("hello: {}", hello);
}
