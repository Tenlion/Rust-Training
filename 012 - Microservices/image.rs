use std::{thread, time};
use std::fs;

fn main() {

    // Initializing constants.
    // Could we have used the walkdir crate?  Yes.  Do I know anything about that crate?  No.
    // Hence, we're going to just ucase a constant for the number of images instead of
    // dynamically getting the amount of files in the directory.
    const NUMBER_OF_IMAGES: u8 = 5;
    const FILE_PATH_TO_READ_FROM: &str = "../../txt/image.txt";
    const FILE_PATH_TO_WRITE_TO: &str = "../../txt/image.txt";
    const DIRECTORY_TO_COLLECT_FILES_FROM: &str = "../../img";

    // Initializing a blank string to hold the file name for later.
    let mut file_name: String = String::from("DIDN'T WORK!");

    // Infinite loop to keep program running.
    loop {

        // Getting the contents of the file and the byte count.
        let file_contents: String = fs::read_to_string(FILE_PATH_TO_READ_FROM).expect("Couldn't handle Result.");
        let byte_count_of_file_content = file_contents.capacity();

        // Checking the byte count to determine if a number or path is present in the file.
        // If a number is present, then a file path will be generated.
        // If a path is present, nothing will happen.
        if (byte_count_of_file_content <= 2) && (byte_count_of_file_content > 0) {

            // Obtaining the file index of where the image is stored in the directory.
            let mut file_index: u8 = fs::read_to_string(FILE_PATH_TO_READ_FROM)
                .expect("Why is this EXPECT NECESSARY?!")     // Had to put another expect here because Rust compiler said so, I hate it.
                .trim()                                             // Getting rid of the newline character from the read operation.
                .parse()                             // Converting the string to a u8.
                .expect("Invalid number in image.txt");              // Handling the Result.

            // Determining if the randomly generated file index value exceeds the number of images.
            // If so, we cut the size down to 0 through moduloing the file index by the number of images.
            // As for why we're always going to set it to 0 through moduloing instead of assigning it to 0...
            // I have no idea why.  I just followed the instructions.
            if file_index > NUMBER_OF_IMAGES {
                file_index %= NUMBER_OF_IMAGES;
            }

            // Looping through the directory until we hit the file index we retrieved.
            let mut counter: u8 = 0;
            let files = fs::read_dir(DIRECTORY_TO_COLLECT_FILES_FROM).unwrap();
            for file in files {
                if counter == file_index {
                    file_name = file.unwrap().path().display().to_string();
                }
                counter += 1;
            }

            // Replacing the content entirety of the image.txt file with the retrieved file name.
            fs::write(FILE_PATH_TO_WRITE_TO, &file_name);
        }
    }
}
