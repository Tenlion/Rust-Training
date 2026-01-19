fn main() {

    // Testing Mutable Reference
    let (mut x, y) = (1, 2);
    add(&mut x, y);
    println!("x: {}, y: {}", x, y);

    // Testing MIN Usage
    println!("word: {}", i8::MIN);

    // Testing String Concatenation
    let new_word: String = String::from("Hello, my name is");
    let other_word: String = String::from("Kira Ash Stephenson.");
    let mut combined_word: String = format!("{} {}", new_word, other_word);
    println!("new word: {}", new_word);
    println!("other word: {}", other_word);
    println!("combined_word: {}", combined_word);

    // Testing Mutable Reference
    string_change(&mut combined_word, &other_word);
    println!("combined_word: {}", combined_word);

    // Testing String Slices
    let s: String = String::from("Hello world!");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}, world: {}", hello, world);

    // Testing Int Slices
    let nums: [i16; 10] = [1,2,3,4,5,6,7,8,9,10];
    let num_slice_1: &[i16] = &nums[0..4];
    let num_slice_2: &[i16] = &nums[7..10];
    println!("num_slice_1: {:?}, num_slice_2: {:?}", num_slice_1, num_slice_2);

    // Testing Looping through a Collection
    let blah: [i16; 5] = [1,2,3,4,5];
    for i in &blah {
        println!("i: {}", i);
    }
}

fn add(x: &mut i32, y: i32) {
    *x += y;
}

fn string_change(s: &mut String, s1: &str) {
    *s = format!("{} {}", s, s1);
}