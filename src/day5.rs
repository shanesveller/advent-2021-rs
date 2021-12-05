use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

type Number = u16;
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: Number,
    y: Number,
}

#[derive(Clone, Copy, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Missing one or both points")]
    MissingPoint,
    // #[error("Missing separator")]
    // MissingSeparator,
    #[error("Malformed line")]
    Malformed,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(',').collect::<Vec<&str>>()[..] {
            [x, y] => Ok(Self {
                x: Number::from_str(x).unwrap(),
                y: Number::from_str(y).unwrap(),
            }),
            _ => Err(Error::MissingPoint),
        }
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(" -> ").collect::<Vec<&str>>()[..] {
            [p1, p2] => Ok(Self {
                p1: Point::from_str(p1)?,
                p2: Point::from_str(p2)?,
            }),
            _ => Err(Error::Malformed),
        }
    }
}

#[derive(Debug)]
struct Map {
    lines: Vec<Line>,
    height: Number,
    width: Number,
    coordinates: HashMap<Point, usize>,
}

impl Map {
    fn new(lines: Vec<Line>) -> Self {
        let mut points = Vec::with_capacity(lines.len() * 2);

        for line in lines.iter() {
            points.push(line.p1);
            points.push(line.p2);
        }

        let (height, width) = points
            .iter()
            .fold((0, 0), |(x, y), point| (x.max(point.x), y.max(point.y)));

        Self {
            height,
            lines,
            width,
            coordinates: HashMap::with_capacity((height * width).into()),
        }
    }

    fn map_lines(&mut self) {
        for line in self
            .lines
            .iter()
            .filter(|l| l.is_horizontal() || l.is_vertical())
        {
            let (origin, target) = if line.p1 < line.p2 {
                (line.p1, line.p2)
            } else {
                (line.p2, line.p1)
            };

            if line.is_vertical() {
                let x = line.p1.x;
                for y in origin.y..=target.y {
                    let count = self.coordinates.entry(Point { x, y }).or_insert(0);
                    *count += 1;
                }
            } else if line.is_horizontal() {
                let y = line.p1.y;
                for x in origin.x..=target.x {
                    let count = self.coordinates.entry(Point { x, y }).or_insert(0);
                    *count += 1;
                }
            }
        }
    }

    fn points_with_min_count(&self, min_count: usize) -> usize {
        self.coordinates
            .iter()
            .filter(|(_point, count)| **count >= min_count)
            .count()
    }
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(Line::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &[Line]) -> usize {
    let mut map = Map::new(input.to_vec());
    map.map_lines();
    map.points_with_min_count(2)
}
