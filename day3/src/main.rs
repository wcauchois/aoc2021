use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILENAME: &str = "input.txt";

fn bits_to_dec(bits: &Vec<bool>) -> i32 {
    i32::from_str_radix(
        &bits
            .into_iter()
            .map(|b| if *b { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join(""),
        2,
    )
    .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rows: Vec<Vec<bool>> = Vec::new();
    {
        let file = File::open(INPUT_FILENAME)?;
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            let line = line?;
            let cols = line.chars().into_iter().map(|c| c == '1').collect();
            rows.push(cols);
        }
    }

    let num_cols = rows[0].len();
    let mut freq_maps: Vec<HashMap<bool, i32>> = (0..num_cols).map(|_| HashMap::new()).collect();
    for col in 0..num_cols {
        for row in &rows {
            let item = row[col];
            *freq_maps[col].entry(item).or_insert(0) += 1;
        }
    }

    let gamma_rate_bits: Vec<bool> = (0..num_cols)
        .map(|col| {
            *freq_maps
                .get(col)
                .unwrap()
                .into_iter()
                .max_by_key(|(_, cnt)| *cnt)
                .unwrap()
                .0
        })
        .collect();

    let epsilon_rate_bits: Vec<bool> = (0..num_cols)
        .map(|col| {
            *freq_maps
                .get(col)
                .unwrap()
                .into_iter()
                .min_by_key(|(_, cnt)| *cnt)
                .unwrap()
                .0
        })
        .collect();

    let gamma_rate = bits_to_dec(&gamma_rate_bits);
    let epsilon_rate = bits_to_dec(&epsilon_rate_bits);

    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);

    println!("Product: {}", gamma_rate * epsilon_rate);

    Ok(())
}
