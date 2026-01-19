
use std::collections::HashMap;

fn main() {

    // Showcasing memory growth (capacity growth) of string with no initial capacity set.
    let mut new_thing: String = String::from("New");
    println!("{}", new_thing.capacity());
    for _ in 0..10 {
        new_thing.push_str("New");
        println!("{}", new_thing.capacity());
    }
    println!();


    // Showcasing memory growth (capacity growth) of string with an initialized capacity being set.
    let mut other_thing: String = String::with_capacity(33);
    other_thing.push_str("New");
    println!("{}", other_thing.capacity());
    for _ in 0..10 {
        other_thing.push_str("New");
        println!("{}", other_thing.capacity());
    }
    println!();


    // Testing out vectors.
    let mut new_vec_0: Vec<i8> = vec![2, 4, 6, 8, 10];
    let mut new_vec_1: Vec<&str> = vec!["Blah1", "Blah78", "Blah083"];
    for _ in 0..10 {
        let thing = new_vec_0.pop();
        match thing {
            Some(number) => println!("{}", number),
            None => println!("Nothing to pop!"),
        };

        new_vec_1.push("I'm a new blah.");
    }
    println!("{:?}", new_vec_1);
    println!();


    // Testing out hashmap inserts.
    let mut map: HashMap<&str, u8> = HashMap::new();
    map.insert("blah", 3);
    map.insert("blah78", 7);
    map.insert("blah083", 8);
    map.entry("blah33").or_insert(33);
    map.entry("blah78").or_insert(54);
    println!("{:?}", map);
    println!();


    // Testing out various hashmap methods.
    map.remove("blah");
    let value = match map.get("blah083") {
        Some(number) => number,
        None => &0,
    };
    println!("{}", value);
}
