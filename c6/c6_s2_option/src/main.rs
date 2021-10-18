fn main() {
    let x = 9;
    println!("x = {}", x);

    let y: Option<u32> = None;
    let z: Option<u32> = Some(100);

    assert_eq!(y.is_some(), false);
    assert_eq!(z.is_some(), true);
    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {:?}", text);
    assert_eq!(text_length.is_some(), true);
    y.expect("Nothing in y");
}
