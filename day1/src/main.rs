use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("input_part1.txt")?;
    let nums = lines.into_iter().map(|l| l.parse::<i32>()).collect::<Result<Vec<i32>, _>>()?;
    let mut num_increases = 0;
    for subslice in nums.into_iter().as_slice().windows(2) {
        match subslice {
            [prev, next] => {
                if next > prev {
                    num_increases += 1;
                }
            },
            _ => panic!(),
        }
    }
    println!("Number of increases: {}", num_increases);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    io::BufReader::new(file).lines().collect::<Result<Vec<String>, _>>()
}
