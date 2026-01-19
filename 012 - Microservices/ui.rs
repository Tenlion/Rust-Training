use std::{thread, time};
use std::fs;
use std::io;

fn main() {

    // Initializing constants.
    const FILE_PATH_FOR_PRNG_TXT: &str = "../../txt/prng.txt";
    const FILE_PATH_FOR_IMAGE_TXT: &str = "../../txt/image.txt";

    loop {

        // Obtaining input from the user to determine if the loop should continue or not.
        let mut input = String::new();
        println!("Please enter 1 to obtain a random image file path, and enter anything else if you wish to stop the program. ");
        std::io::stdin().read_line(&mut input).unwrap();


        if input == "1\n" {

            // "run" is used to initiate the prng service.  It does this since the prng program
            // is constantly looking for the word "run" inside the prng.txt file to initiate its code.
            fs::write(FILE_PATH_FOR_PRNG_TXT, "run");

            // Loading message is present to notify that values in text files are changing before
            // allowing the UI program to run and further manipulate the text files.
            println!("Processing 1/6...");
            thread::sleep(time::Duration::from_secs(1));
            println!("Processing 2/6...");
            thread::sleep(time::Duration::from_secs(1));
            println!("Processing 3/6...");
            thread::sleep(time::Duration::from_secs(1));

            // Getting the index of the file from prng.txt for the image service to use.
            let file_index: u8 = fs::read_to_string(FILE_PATH_FOR_PRNG_TXT)
                .expect("Why is this EXPECT NECESSARY?!")     // Had to put another expect here because Rust compiler said so, I hate it.
                .trim()                                             // Getting rid of the newline character from the read operation.
                .parse()                             // Converting the string to a u8.
                .expect("Invalid number in prng.txt.");              // Handling the Result.

            // Writes the index of the file to image.txt to initiate the image service.
            fs::write(FILE_PATH_FOR_IMAGE_TXT, file_index.to_string());

            // Loading message is present to notify that values in text files are changing before
            // allowing the UI program to run and further manipulate the text files.
            println!("Processing 4/6...");
            thread::sleep(time::Duration::from_secs(1));
            println!("Processing 5/6...");
            thread::sleep(time::Duration::from_secs(1));
            println!("Processing 6/6...");
            thread::sleep(time::Duration::from_secs(1));
            println!();

            // Obtaining and printing the generated file path.
            let file_path: String = fs::read_to_string(FILE_PATH_FOR_IMAGE_TXT).expect("Couldn't handle Result.");
            println!("Relative File Path: {}", file_path);
            println!();
        }
        else {

            // Tells the infinite loop to stop.  This will ultimately close the program.
            break;
        }
    }


}
