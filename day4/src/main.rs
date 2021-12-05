use std::cell::{Cell, RefCell};
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILENAME: &str = "input.txt";

const BOARD_SIZE: usize = 5;
const BOARD_COUNT: usize = BOARD_SIZE * BOARD_SIZE;

#[derive(Debug)]
struct Board {
    nums: [i32; BOARD_COUNT],
    marked: RefCell<[bool; BOARD_COUNT]>,
    frozen: Cell<bool>,
}

impl Board {
    fn empty() -> Self {
        Board {
            nums: [0; BOARD_COUNT],
            marked: RefCell::new([false; BOARD_COUNT]),
            frozen: Cell::new(false),
        }
    }

    fn mark(&self, num: i32) {
        if self.frozen.get() {
            return;
        }
        for i in 0..BOARD_COUNT {
            if self.nums[i] == num {
                self.marked.borrow_mut()[i] = true;
            }
        }
    }

    fn wins(&self) -> bool {
        for by_rows in [true, false] {
            for i in 0..BOARD_SIZE {
                if (0..BOARD_SIZE).all(|j| match by_rows {
                    true => self.marked.borrow()[i * BOARD_SIZE + j],
                    false => self.marked.borrow()[j * BOARD_SIZE + i],
                }) {
                    return true;
                }
            }
        }
        false
    }

    fn freeze(&self) {
        self.frozen.set(true);
    }

    fn score(&self, called_num: i32) -> i32 {
        let unmarked_total: i32 = (0..BOARD_COUNT)
            .map(|i| {
                if !self.marked.borrow()[i] {
                    self.nums[i]
                } else {
                    0
                }
            })
            .sum();
        unmarked_total * called_num
    }
}

#[derive(Debug)]
struct PuzzleInput {
    drawn_numbers: Vec<i32>,
    boards: Vec<Board>,
}

impl PuzzleInput {
    fn read_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();

        let mut drawn_numbers: Vec<i32> = Vec::new();
        let mut boards: Vec<Board> = Vec::new();

        enum ParseState {
            Init,
            Board { line_num: i32 },
        }

        let mut parse_state = ParseState::Init;
        for line in lines {
            let line = line?;
            match parse_state {
                ParseState::Init if line != "" => {
                    drawn_numbers = line.split(",").map(|s| s.parse().unwrap()).collect();
                }
                ParseState::Init => {
                    parse_state = ParseState::Board { line_num: 0 };
                }
                ParseState::Board { line_num } if line != "" => {
                    if line_num == 0 {
                        boards.push(Board::empty());
                    }
                    let board = boards.last_mut().unwrap();
                    let row: Vec<i32> = line
                        .split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                    board.nums
                        [(line_num as usize * BOARD_SIZE)..((line_num + 1) as usize * BOARD_SIZE)]
                        .clone_from_slice(row.as_slice());
                    let line_num = line_num + 1;
                    parse_state = ParseState::Board {
                        line_num: if line_num >= BOARD_SIZE as i32 {
                            0
                        } else {
                            line_num
                        },
                    };
                }
                ParseState::Board { .. } => {}
            }
        }

        Ok(PuzzleInput {
            boards,
            drawn_numbers,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = PuzzleInput::read_from_file(INPUT_FILENAME)?;
    // println!("Puzzle input: {:#?}", puzzle_input);

    let mut last_winner: Option<(&Board, i32)> = None;
    for drawn_num in puzzle_input.drawn_numbers {
        for board in &puzzle_input.boards {
            board.mark(drawn_num);
            if board.wins() && !board.frozen.get() {
                board.freeze();
                last_winner = Some((board, drawn_num));
            }
        }
    }

    if let Some((board, drawn_num)) = last_winner {
        println!("Board is: {:?}", board);
        println!("Score: {}", board.score(drawn_num));
    }

    Ok(())
}
