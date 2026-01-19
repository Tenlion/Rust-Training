use std::fs;
use rand::Rng;

fn main() {

    // Initializing constants.
    const RAND_MAX: u8 = 5;
    const RAND_MIN: u8 = 0;
    const FILE_PATH_TO_WRITE_TO: &str = "../../txt/prng.txt";
    const FILE_PATH_TO_READ_FROM: &str = "../../txt/prng.txt";

    // Initiating a random instance to manipulate in the loop.
    let mut randomizer = rand::rng();

    // Infinite loop to keep program running.
    loop {

        let file_contents: String = fs::read_to_string(FILE_PATH_TO_READ_FROM).expect("Couldn't handle Result.");

        if file_contents == "run" {

            // Generating a random number.
            let random_number: u8 = randomizer.random_range(RAND_MIN..RAND_MAX);

            // Replacing the content entirety of the prng.txt file with the newly generated random number.
            fs::write(FILE_PATH_TO_WRITE_TO, random_number.to_string());
        }
    }
}
