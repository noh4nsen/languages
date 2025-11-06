use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    let team_name = String::from("Purple");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    println!("{:?}", scores);
    scores.insert(String::from("Red"), 140);
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Purple");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);

    //Overwrite
    scores.insert(String::from("Blue"), 600);
    println!("{:?}", scores);

    //Entry
    scores.entry(String::from("Purple")).or_insert(49);
    //Nao altera blue pq valor ja existe
    scores.entry(String::from("Blue")).or_insert(5);
    println!("{:?}", scores);

    //update based on old value
    let text = "hello world wonderful world hello world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);


}
