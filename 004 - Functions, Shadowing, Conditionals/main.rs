fn main() {

    // Testing Shadowing
    let new_thing: i32 = 10;
    let new_thing: i32 = 100;
    println!("The value of new_thing is {}", new_thing);

    // More Shadow Testing
    let x = 2;
    type_of(&x);
    let x: i8 = x * 20;
    type_of(&x);

    // Testing Copy Type
    let y= x;

    // Else If Test
    if (x > y) {
        println!("The value of x is {}", x);
    }
    else if (x < y) {
        println!("The value of y is {}", y);
    }

    // Testing Rust Ternary Operator
    let other_thing: i16 = if (x < y) {3} else {-3};
    println!("The value of other_thing is {}", other_thing);

    // Testing Later Immutable Initialization
    let z;
    z = add(5, 10);

    // Testing Constant
    const W: i8 = 8;

    // Printing Values
    println!("Y Value: {y}");
    println!("X Value: {x}");
    println!("Z Value: {z}");
    println!("W Value: {W}");

    // Testing Immutable Struct Usage
    let new_user = User {
        active: true,
        username: "Blah".to_string(),
        email: "Blah@blah.com".to_string(),
        sign_in_count: 10,
    };
    println!("User Active Status: {}", new_user.active);
    println!("Username: {}", new_user.username);
    println!("User Email: {}", new_user.email);
    println!("User Sign-Ins: {}", new_user.sign_in_count);
}

fn add(num_1: i8, num_2: i8) -> i8 {
    return num_1 + num_2;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}