
fn main() {

    // Testing Generics
    let new_rgb_0: RGB<i8> = RGB::new(16, 16, 16);
    println!("New RGB: {:?}", new_rgb_0);
    println!("Byte Value of Fields: {}", size_of_val(&new_rgb_0));
    println!();

    let new_rgb_1: RGB<u16> = RGB::new(255, 255, 255);
    println!("New RGB: {:?}", new_rgb_1);
    println!("Byte Value of Fields: {}", size_of_val(&new_rgb_1));
    println!();

    let new_rgb_2: RGB<i32> = RGB::new(0, 23442, -233);
    println!("New RGB: {:?}", new_rgb_2);
    println!("Byte Value of Fields: {}", size_of_val(&new_rgb_2));
    println!();

    let new_rgb_3: RGB<&'static str> = RGB::new("reddish", "light green", "not much blue");
    println!("New RGB: {:?}", new_rgb_3);
    println!("Byte Value of Fields: {}", size_of_val(&new_rgb_3));
    println!();


    // Testing a Static Trait
    let new_bullshit_0: RandomBullshitGo<u8, f32, &'static str> = RandomBullshitGo::new(3, 3.328, "Fuck you.");
    new_bullshit_0.print_contents();

    let new_rgb_4: RGB<u8> = RGB::new(3, 4, 38);
    new_rgb_4.print_contents();

    let new_bullshit_1: RandomBullshitGo<i32, f64, &str> = RandomBullshitGo::new(-32, 3.888383883838838, "I love this!");
    new_bullshit_1.print_contents();

    let new_rgb_5: RGB<i128> = RGB::new(300000000, -34298347, 0000);
    new_rgb_5.print_contents();
}

#[derive(Debug)]
struct RGB<TYPE> {
    r: TYPE,
    g: TYPE,
    b: TYPE,
}

struct RandomBullshitGo<T, U, W> {
    num_1: T,
    num_2: U,
    num_info: W,
}

impl<TYPE> RGB<TYPE> {
    fn new(r: TYPE, g: TYPE, b: TYPE) -> Self {
        RGB { r, g, b }
    }
}

impl<T, U, W> RandomBullshitGo<T, U, W> {
    fn new(num_1: T, num_2: U, num_info: W) -> Self {
        RandomBullshitGo { num_1, num_2, num_info }
    }
}

trait Printable {
    fn print_contents(&self);
}

impl<TYPE: std::fmt::Display> Printable for RGB<TYPE> {
    fn print_contents(&self) {
        println!("RGB VALUE: {}, {}, {}", self.r, self.g, self.b);
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display, W: std::fmt::Display> Printable for RandomBullshitGo<T, U, W> {
    fn print_contents(&self) {
        println!("CONTENTS OF RANDOM BULLSHIT: {}, {}, \"{}\"" , self.num_1, self.num_2, self.num_info);
    }
}