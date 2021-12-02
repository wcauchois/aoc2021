use nom::branch::alt;
use nom::character::complete::{digit1, space1};
use nom::combinator::value;
use nom::{bytes::complete::tag, combinator::map_res, IResult};
use std::fs::File;
use std::io::{self, BufRead};

const INPUT_FILENAME: &str = "input.txt";

#[derive(Debug)]
struct Position {
    horiz: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            horiz: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn with_horiz(&self, horiz: i32) -> Position {
        Position {
            horiz,
            depth: self.depth,
            aim: self.aim,
        }
    }

    fn with_depth(&self, depth: i32) -> Position {
        Position {
            horiz: self.horiz,
            depth,
            aim: self.aim,
        }
    }

    fn with_aim(&self, aim: i32) -> Position {
        Position {
            horiz: self.horiz,
            depth: self.depth,
            aim,
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
            CommandType::Forward => pos
                .with_horiz(pos.horiz + self.amount)
                .with_depth(pos.depth + pos.aim * self.amount),
            CommandType::Up => pos.with_aim(pos.aim - self.amount),
            CommandType::Down => pos.with_aim(pos.aim + self.amount),
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
        Command {
            command_type,
            amount,
        },
    ))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(INPUT_FILENAME)?;
    let lines = io::BufReader::new(file).lines();
    let mut pos = Position::new();
    for line in lines {
        let line = line?;
        let (_, command) = parse_command(&line)
            .map_err(|_e| io::Error::new(io::ErrorKind::Other, "Failed to parse"))?;
        pos = command.applied_to(pos);
    }
    println!("Result: {}", pos.horiz * pos.depth);
    Ok(())
}
