fn main() {

    // Testing variabled ranges in for loops.
    let y: i8 = 3;
    let x: i8 = 7;
    for i in y..=x {
        println!("{}", i);
    }

    // Testing temporary shadow usage.
    let mut z: i8 = 12;
    {
        let z = z + 3;
        println!("The value of z is {}", z);
    }
    println!("The value of z is {}", z);


    // Testing continue on labels and testing labels on for loops.
    'outer: for mut i in 0..10 {
        println!("");
        println!("I Value: {i}");
        'inner: for mut k in 0..5 {
            println!("K Value: {k}");
            if k == 3 {
                continue 'outer;
            }
        }
        println!("Hello, world!");
    }

    // Testing out structs with impl usage.
    let new_desk: Desk = build_desk("Kira's Desk".to_string(), 5, "Large Tower".to_string(), "32-Inch Monitor".to_string());
    let another_desk: Desk = build_desk("Gabe's Desk".to_string(), 0, "His Body".to_string(), "Gabe's Eyeballs".to_string());
    println!();
    another_desk.list_fields();
    println!();
    new_desk.list_fields();

    // Testing out a tuple struct.
    let mut new_color: Color = Color(0, 0, 0);
    new_color.0 = 255;
    print!("{} {} {} \n", new_color.0, new_color.1, new_color.2);
}


struct Desk {
    name: String,
    legs: i8,
    computer: String,
    monitor: String,
}

struct Color(i16, i16, i16);

fn build_desk(name: String, legs: i8, computer: String, monitor: String) -> Desk {
    Desk {
        name,
        legs,
        computer,
        monitor,
    }
}

impl Desk {
    fn list_fields(&self) {
        println!("Name of Desk: {}", self.name);
        println!("Leg Count: {}", self.legs);
        println!("Computer on Desk: {}", self.computer);
        println!("Monitor on Desk: {}", self.monitor);
    }
}
