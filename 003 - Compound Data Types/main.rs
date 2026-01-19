fn main() {

    // Array Testing
    let numbers: [i8; 3] = [1, 2, 3];
    println!("The value of numbers: {:?}", numbers);
    println!("{}", numbers[0]);
    println!("{}", numbers[1]);
    println!("{}", numbers[2]);

    let stuff: [&str; 3] = ["First", "Second", "Third"];
    println!("The Array: {:?}", stuff);
    println!("{}", stuff[0]);
    println!("{}", stuff[1]);
    println!("{}", stuff[2]);

    // Tuple Testing
    let human: (String, i32, bool) = ("Alice".to_string(), 35, true);
    println!("Human Tuple: {:?}", human);

    let mixed_things: (i8, String, [i8; 3]) = (5, "Yo".to_string(), [35, 33, 31]);
    let (number, text, collection_of_numbers) = mixed_things;
    println!("Thing A: {}", number);
    println!("Thing B: {}", text);
    println!("Thing C: {:?}", collection_of_numbers);

    // Slice Testing
    let number_slices :&[i16] = &[-3, 0, -100, 34, 78];
    println!("Number Slice: {:?}", number_slices);

    let animal_slices :&[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slices :&[&String] = &[&"Book 1".to_string(), &"Book 2".to_string()];
    println!("Book Slice: {:?}", book_slices);

    // Strings vs. String Slices (&str)
    // String Type
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("yeah!");
    println!("Stone Cold says: {}", stone_cold);

    // String Slice Type
    let min :i8 = 0;
    let max :i8 = 4;
    let string: String = String::from("😂 hi my name is blah");
    let slice: &str = &string[min as usize..max as usize];
    println!("Slice Value: {}", slice);
    println!("I'm using number {} for my min and number {} for my max.", min, max);
}
