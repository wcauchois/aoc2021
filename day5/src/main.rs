use std::cmp::{min, max};
use std::fs::File;
use std::io::{self, BufRead};
use nom::character::complete::space1;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::{character::complete::digit1, IResult, bytes::complete::tag, combinator::map_res, sequence::tuple};

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    point1: Point,
    point2: Point,
}

#[derive(Debug)]
struct PuzzleInput {
    lines: Vec<Line>,
}

#[derive(Debug)]
struct BoardCell {
    num_crosses: i32,
}

impl BoardCell {
    fn new() -> Self {
        BoardCell { num_crosses: 0 }
    }
}

#[derive(Debug)]
struct Board {
    cells: Vec<BoardCell>,
    width: i32,
    height: i32,
}

impl Board {
    fn new_with_capacity_for_lines(lines: &Vec<Line>) -> Self {
        let width = lines.into_iter().flat_map(|l| vec![l.point1.x, l.point2.x]).max().unwrap();
        let height = lines.into_iter().flat_map(|l| vec![l.point1.y, l.point2.y]).max().unwrap();
        let capacity = (width + 1) * (height + 1);
        Board {
            width,
            height,
            cells: (0..capacity).map(|_| BoardCell::new()).collect()
        }
    }

    fn get_cell(&self, x: i32, y: i32) -> &BoardCell {
        self.cells.get((x * self.width + y) as usize).unwrap()
    }

    fn get_cell_mut(&mut self, x: i32, y: i32) -> &mut BoardCell {
        self.cells.get_mut((x * self.width + y) as usize).unwrap()
    }

    fn capacity(&self) -> i32 {
        self.width * self.height
    }
}

impl Line {
    fn apply_to(&self, board: &mut Board) {
        if self.point1.x == self.point2.x {
            // Vertical line
            for y in min(self.point1.y, self.point2.y)..=max(self.point1.y, self.point2.y) {
                board.get_cell_mut(self.point1.x, y).num_crosses += 1;
            }
        } else if self.point1.y == self.point2.y {
            // Horizontal line
            for x in min(self.point1.x, self.point2.x)..=max(self.point1.x, self.point2.x) {
                board.get_cell_mut(x, self.point1.y).num_crosses += 1;
            }
        } else {
            // panic!("Line was neither horizontal nor vertical {:?}", self);
        }
    }
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(parse_i32, tag(","), parse_i32),
        |(x, y)| Point { x, y }
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, point1) = parse_point(input)?;
    let (input, _) = tuple((space1, tag("->"), space1))(input)?;
    let (input, point2) = parse_point(input)?;
    Ok((input, Line { point1, point2 }))
}

impl PuzzleInput {
    fn read_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let str_lines = io::BufReader::new(file).lines();

        let mut lines: Vec<Line> = Vec::new();

        for str_line in str_lines {
            let str_line = str_line?;
            let (_, line) = parse_line(&str_line).map_err(|_e| {
                io::Error::new(io::ErrorKind::Other, "Failed to parse")
            })?;
            lines.push(line);
        }

        Ok(PuzzleInput { lines })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let puzzle_input = PuzzleInput::read_from_file(INPUT_FILENAME)?;
    // println!("Puzzle input: {:#?}", puzzle_input);

    let mut board = Board::new_with_capacity_for_lines(&puzzle_input.lines);
    for line in &puzzle_input.lines {
        line.apply_to(&mut board)
    }

    let num_overlapping_points = board.cells.into_iter().filter(|c| c.num_crosses > 1).count();
    println!("Num overlaps: {}", num_overlapping_points);

    Ok(())
}
