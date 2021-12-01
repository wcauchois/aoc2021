use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("input_part1.txt")?;
    let nums = lines
        .into_iter()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;
    let mut num_increases = 0;
    for item in nums
        .into_iter()
        .as_slice()
        .windows(3)
        .collect::<Vec<_>>()
        .as_slice()
        .windows(2)
    {
        match item {
            [fst, snd] => {
                let fst_sum = fst.iter().sum::<i32>();
                let snd_sum = snd.iter().sum::<i32>();
                if snd_sum > fst_sum {
                    num_increases += 1;
                }
            }
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
    io::BufReader::new(file)
        .lines()
        .collect::<Result<Vec<String>, _>>()
}
