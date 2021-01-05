use std::collections::HashMap;

pub fn hash() {
    let mut players: HashMap<String, i32> = HashMap::new();

    players.insert(String::from("hello"), 2);

    let names = vec![String::from("john"), String::from("jacob")];
    let age = vec![10, 20];

    let person: HashMap<_, _> = names.into_iter().zip(age.into_iter()).collect();

    println!("{:?}", players);
    println!("{:?}", person);

    match person.get("john") {
        Some(val) => println!("john value: {}", val),
        None => println!("not found"),
    }

    println!("{:?}", players);

    let text = "hello world hello hello";

    let mut countWords: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        println!("{}", word);
        if word == &text[..5] {
            println!("slice matches");
        } 
        let count: &mut i32 = countWords.entry(word).or_insert(0);
        *count += 1;
    }

    countWords.entry("apple").or_insert(99);
    countWords.entry("apple").or_insert(102);

    println!("{:?}", countWords);

}
