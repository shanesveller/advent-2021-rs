use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Number = u8;
type Drawings = Vec<Number>;
type Coord = (usize, usize);

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Clone, Debug, Default)]
struct Board {
    positions: HashMap<Number, Coord>,
    numbers: HashSet<Number>,
    last_drawn: Option<Number>,
    drawn: HashSet<Number>,
    drawn_coords: Vec<Coord>,
}

impl Board {
    fn new(cells: Vec<Number>) -> Self {
        let positions = cells
            .iter()
            .enumerate()
            .map(|(idx, n)| (*n, (idx.rem_euclid(WIDTH) + 1, idx.div_euclid(HEIGHT) + 1)))
            .collect();
        let numbers = cells.into_iter().collect();
        Self {
            positions,
            numbers,
            ..Default::default()
        }
    }

    #[cfg(test)]
    fn clear_draws(&mut self) {
        self.drawn.clear();
        self.drawn_coords.clear();
    }

    fn draw(&mut self, n: Number) {
        let _old = self.last_drawn.insert(n);
        if self.has_number(n) && self.drawn.insert(n) {
            self.drawn_coords.push(self.coord_for_number(n).unwrap());
        }
    }

    fn has_number(&self, n: Number) -> bool {
        self.numbers.contains(&n)
    }

    fn coord_for_number(&self, n: Number) -> Option<Coord> {
        self.positions.get(&n).copied()
    }

    fn score(&self) -> usize {
        let unscored: usize = self
            .numbers
            .difference(&self.drawn)
            .map(|&n| usize::from(n))
            .sum();
        unscored
            .checked_mul(self.last_drawn.unwrap().into())
            .unwrap()
    }

    fn is_won(&self) -> bool {
        if self.drawn.len() >= 5 {
            self.forms_column() || self.forms_row()
        } else {
            false
        }
    }

    fn forms_column(&self) -> bool {
        self.drawn_coords
            .iter()
            .counts_by(|(x, _y)| x)
            .iter()
            .any(|(_col, count)| *count == 5)
    }

    fn forms_row(&self) -> bool {
        self.drawn_coords
            .iter()
            .counts_by(|(_x, y)| y)
            .iter()
            .any(|(_col, count)| *count == 5)
    }
}

#[aoc_generator(day4)]
fn parse_boards_and_drawings(input: &str) -> (Drawings, Vec<Board>) {
    let mut iter = input.lines();

    let drawings = iter
        .next()
        .map(|l| l.split(',').map(|n| str::parse::<u8>(n).unwrap()).collect())
        .unwrap();

    let boards = iter
        .chunks(6)
        .into_iter()
        .map(|board_lines| {
            let numbers: Vec<Number> = board_lines
                .skip(1)
                .flat_map(|l| {
                    l.split_ascii_whitespace()
                        .map(|n| str::parse::<u8>(n).unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect();
            Board::new(numbers)
        })
        .collect();

    (drawings, boards)
}

#[aoc(day4, part1)]
fn find_winning_score(input: &(Drawings, Vec<Board>)) -> usize {
    let mut drawn_boards = input.1.clone();

    let mut winner: Option<Board> = None;
    let mut iter = input.0.iter();

    while winner.is_none() {
        if let Some(drawing) = iter.next() {
            drawn_boards.iter_mut().for_each(|board| {
                board.draw(*drawing);
                if board.is_won() {
                    winner.get_or_insert(board.clone());
                }
            });
        } else {
            break;
        }
    }

    winner.unwrap().score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_for_number() {
        let board = Board::new((1..=25).collect());

        assert_eq!(board.coord_for_number(1), Some((1, 1)));
        assert_eq!(board.coord_for_number(6), Some((1, 2)));
        assert_eq!(board.coord_for_number(15), Some((5, 3)));
        assert_eq!(board.coord_for_number(25), Some((5, 5)));
        assert_eq!(board.coord_for_number(26), None);
    }

    #[test]
    fn test_forms_row() {
        let mut board = Board::new((1..=25).collect());
        (1..=5).for_each(|n| board.draw(n));
        assert!(!board.forms_column());
        assert!(board.forms_row());

        board.clear_draws();

        (1..=25).step_by(5).for_each(|n| board.draw(n));
        assert!(!board.forms_row());
        assert!(board.forms_column());
    }

    #[test]
    fn test_score() {
        let mut board = Board::new(vec![
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]);

        for n in [7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21] {
            board.draw(n);
        }
        assert!(!board.is_won());

        board.draw(24);

        assert!(board.forms_row());
        assert!(board.is_won());
        assert_eq!(board.score(), 4512);
    }
}
