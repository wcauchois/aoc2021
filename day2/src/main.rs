use std::fs::File;
use std::io::{self, BufRead};
use nom::branch::alt;
use nom::character::complete::{digit1, space1};
use nom::combinator::value;
use nom::{bytes::complete::tag, combinator::map_res, IResult};

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct Position {
    horiz: i32,
    depth: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            horiz: 0,
            depth: 0,
        }
    }

    fn add_horiz(&self, amount: i32) -> Position {
        Position {
            horiz: self.horiz + amount,
            depth: self.depth,
        }
    }

    fn add_depth(&self, amount: i32) -> Position {
        Position {
            horiz: self.horiz,
            depth: self.depth + amount,
        }
    }
}

#[derive(Clone, Debug)]
enum CommandType {
    Forward,
    Up,
    Down,
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    amount: i32,
}

impl Command {
    fn applied_to(&self, pos: Position) -> Position {
        match self.command_type {
            CommandType::Forward => pos.add_horiz(self.amount),
            CommandType::Up => pos.add_depth(-self.amount),
            CommandType::Down => pos.add_depth(self.amount),
        }
    }
}

fn parse_command_type(input: &str) -> IResult<&str, CommandType> {
    alt((
        value(CommandType::Forward, tag("forward")),
        value(CommandType::Up, tag("up")),
        value(CommandType::Down, tag("down")),
    ))(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, command_type) = parse_command_type(input)?;
    let (input, _) = space1(input)?;
    let (input, amount) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
    Ok((
        input,
        Command { command_type, amount }
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(INPUT_FILENAME)?;
    let lines = io::BufReader::new(file).lines();
    let mut pos = Position::new();
    for line in lines {
        let line = line?;
        let (_, command) = parse_command(&line).map_err(|_e| io::Error::new(io::ErrorKind::Other, "Failed to parse"))?;
        pos = command.applied_to(pos);
    }
    println!("Result: {}", pos.horiz * pos.depth);
    Ok(())
}
