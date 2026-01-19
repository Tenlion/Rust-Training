
use std::num::ParseIntError;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {

    // Testing From
    let blah = TestingShit::from(-20_i16);
    println!("Data 1 for Blah: {}", blah.data_1);
    println!("Data 2 for Blah: {}", blah.data_2);
    println!("Data 3 for Blah: {}", blah.data_3);
    println!("Data 4 for Blah: {}", blah.data_4);
    println!("Data 5 for Blah: {}", blah.data_5);
    println!("");

    // Testing if the embedded return in the ? operator will stop the main
    // function from running.
    let n1: i32 = "3".parse::<i32>()?;
    let n2: i32 = "word".parse::<i32>()?;
    println!("Wasssss upppp? {}", n1 * n2);

    // Testing Into
    let reverse_blah: TestingShit<String> = String::from("Blahhhhhhh").into();
    println!("Data 1 for ReverseBlah: {}", reverse_blah.data_1);
    println!("Data 2 for ReverseBlah: {}", reverse_blah.data_2);
    println!("Data 3 for ReverseBlah: {}", reverse_blah.data_3);
    println!("Data 4 for ReverseBlah: {}", reverse_blah.data_4);
    println!("Data 5 for ReverseBlah: {}", reverse_blah.data_5);

    assert_eq!(multiply("3", "4").unwrap(), 12);

    Ok(())
}

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1: i32 = n1_str.parse::<i32>()?;
    let n2: i32 = n2_str.parse::<i32>()?;

    println!("Testing if this gets read.");

    Ok(n1 * n2)
}

struct TestingShit<T> {
    data_1: T,
    data_2: T,
    data_3: T,
    data_4: T,
    data_5: T
}

// FROM IMPLEMENT for any type!
impl<T: Clone> From<T> for TestingShit<T> {
    fn from(value: T) -> TestingShit<T> {
        TestingShit {
            data_1: value.clone(),
            data_2: value.clone(),
            data_3: value.clone(),
            data_4: value.clone(),
            data_5: value.clone()
        }
    }
}
