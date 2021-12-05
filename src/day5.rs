use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};

type Number = i32;
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: Number,
    y: Number,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
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
    #[error("Malformed line")]
    Malformed,
}

enum Slope {
    Horizontal,
    Vertical,
    SouthwestNortheast,
    NorthwestSoutheast,
}

#[derive(PartialEq)]
enum ProblemPart {
    Part1,
    Part2,
}

struct LinePoints {
    next: Point,
    reached: bool,
    target: Point,
    slope: Slope,
}

impl Line {
    fn points(&self) -> LinePoints {
        let (origin, target) = if self.p1 < self.p2 {
            (self.p1, self.p2)
        } else {
            (self.p2, self.p1)
        };
        LinePoints {
            next: origin,
            reached: origin == target,
            slope: self.slope(),
            target,
        }
    }
    fn slope(&self) -> Slope {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;

        if dx == 0 {
            Slope::Vertical
        } else if dy == 0 {
            Slope::Horizontal
        } else if dy / dx > 0 {
            Slope::SouthwestNortheast
        } else {
            Slope::NorthwestSoutheast
        }
    }
}

impl Iterator for LinePoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reached {
            None
        } else {
            let delta = match self.slope {
                Slope::Horizontal => Point { x: 1, y: 0 },
                Slope::Vertical => Point { x: 0, y: 1 },
                Slope::NorthwestSoutheast => Point { x: 1, y: -1 },
                Slope::SouthwestNortheast => Point { x: 1, y: 1 },
            };
            let emit = self.next;
            self.reached = self.next == self.target;
            self.next += delta;
            Some(emit)
        }
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
        let (height, width) = lines.iter().fold((0, 0), |(x, y), line| {
            (
                x.max(line.p1.x).max(line.p2.x),
                y.max(line.p1.y).max(line.p2.y),
            )
        });

        Self {
            height,
            lines,
            width,
            coordinates: HashMap::with_capacity((height * width).try_into().unwrap()),
        }
    }

    fn map_lines(&mut self, problem_part: ProblemPart) {
        for line in self.lines.iter().filter(|l| match l.slope() {
            Slope::Horizontal | Slope::Vertical => true,
            _ => problem_part != ProblemPart::Part1,
        }) {
            for point in line.points() {
                let count = self.coordinates.entry(point).or_insert(0);
                *count += 1;
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
    map.map_lines(ProblemPart::Part1);
    map.points_with_min_count(2)
}

#[aoc(day5, part2)]
fn part2(input: &[Line]) -> usize {
    let mut map = Map::new(input.to_vec());
    map.map_lines(ProblemPart::Part2);
    map.points_with_min_count(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_points() {
        let line = Line {
            p1: Point { x: 1, y: 1 },
            p2: Point { x: 3, y: 3 },
        };
        assert_eq!(
            line.points().collect::<Vec<Point>>(),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 },
            ]
        )
    }
}
