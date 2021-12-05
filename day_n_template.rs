use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILENAME: &str = "testinput.txt";

#[derive(Debug)]
struct PuzzleInput {
}

impl PuzzleInput {
    fn read_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();

        for line in lines {
            let line = line?;
            // TODO
        }

        panic!(); // TODO
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = PuzzleInput::read_from_file(INPUT_FILENAME)?;
    println!("Puzzle input: {:#?}", puzzle_input);

    Ok(())
}
