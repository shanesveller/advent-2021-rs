use aoc_runner_derive::{aoc, aoc_generator};
use std::{ops::AddAssign, str::FromStr};

#[derive(Debug)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bad direction")]
    BadDirection,
    #[error("Bad magnitude")]
    BadMagnitude,
    #[error("Malformed instruction")]
    BadInstruction,
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(Self::Err::BadDirection),
        }
    }
}

#[derive(Debug)]
pub struct Instruction(Direction, usize);

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
            &[dir, mag] => {
                let direction = Direction::from_str(dir)?;
                let magnitude = str::parse::<usize>(mag).map_err(|_| Self::Err::BadMagnitude)?;
                Ok(Self(direction, magnitude))
            }
            _ => Err(Self::Err::BadInstruction),
        }
    }
}

#[derive(Debug)]
struct Coordinates(usize, usize);

impl AddAssign<&Instruction> for Coordinates {
    fn add_assign(&mut self, rhs: &Instruction) {
        match rhs.0 {
            Direction::Forward => self.1 += rhs.1,
            Direction::Down => self.0 += rhs.1,
            Direction::Up => self.0 -= rhs.1,
        }
    }
}

#[aoc_generator(day2)]
pub fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<Instruction>, Error>>()
        .unwrap()
}

#[aoc(day2, part1)]
pub fn sum(input: &[Instruction]) -> usize {
    let final_pos = input.iter().fold(Coordinates(0, 0), |mut coord, instr| {
        coord += instr;
        coord
    });

    final_pos.0 * final_pos.1
}
