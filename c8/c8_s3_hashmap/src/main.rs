use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];
    let another_scores: HashMap<String, _> =
        teams.into_iter().zip(init_scores.into_iter()).collect();

    println!("{:?}", another_scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    
    let score_of_blue = match scores.get("Blue") {
        Some(x) => x,
        None => &0
    };
    println!("blue's score is {:?}", score_of_blue);

    for (k, v) in &scores {
        println!("{}'s score is {}", k, v);
    }
    scores.insert(String::from("Blue"), 25);
    for (k, v) in &scores {
        println!("{}'s score is {}", k, v);
    }

    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("Purple")).or_insert(60);
    println!("{:?}", scores);

    let text = "hello world another wonderful world";
    let mut text_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = text_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", text_map);
}
