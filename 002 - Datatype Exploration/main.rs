fn main() {

    // Signed and Unsigned Integer Primitives
    let x: i8 = -5;
    let y: u64 = 32;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Float Primitives
    let pi: f64  = 3.141592653589793;
    let e: f32 = 2.7182818;
    println!("Double: {}", pi);
    println!("Float: {}", e);

    // Boolean Primitive
    let is_true: bool = true;
    let is_false: bool = false;
    println!("Are planets flat? {}", is_false);
    println!("Are planets spherical? {}", is_true);

    // Character Primitive
    let char_z: char = 'z';
    let char_exclamation: char = '!';
    println!("The Character 'z': {}", char_z);
    println!("The Character '!': {}", char_exclamation);
}
