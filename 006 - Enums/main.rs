fn main() {

    let mut blah_thing: Blah = Blah::SuperBlah{x: 5, y: 10, z: 20};

    match blah_thing {
        Blah::SuperBlah{ref mut x, ..} => *x = 12,
        Blah::UltraBlah{ref mut a, ..} => *a = 3,
        _ => println!("no match")
    }

    match blah_thing {
        Blah::SuperBlah{x, y, z} => println!("x: {}, y: {}, z: {}", x, y, z),
        Blah::UltraBlah{a, b, c} => println!("a: {}, b: {}, c: {}", a, b, c),
        _ => println!("no match")
    }

    let new_thing: &str = "This is a string slice.";
    let mut other_thing = new_thing;
    other_thing = "New content.";
    println!("{}", new_thing);
    println!("{}", other_thing);
}

enum Blah {
    SuperBlah {x: i8, y: i8, z: i8},
    UltraBlah {a: i8, b: i8, c: i8},
}
