use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Coord = (isize, isize);
type Number = usize;

#[derive(Debug)]
struct Input {
    grid: HashMap<Coord, Number>,
    height: isize,
    width: isize,
}

#[aoc_generator(day9)]
fn input_parser(input: &str) -> Input {
    let grid: HashMap<Coord, Number> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    (
                        (x.try_into().unwrap(), y.try_into().unwrap()),
                        Number::try_from(c.to_digit(10).unwrap()).unwrap(),
                    )
                })
                .collect::<Vec<(Coord, Number)>>()
        })
        .collect();

    let (max_x, max_y) = grid.keys().fold((0, 0), |(old_x, old_y), (x, y)| {
        (old_x.max(*x), old_y.max(*y))
    });

    Input {
        grid,
        height: max_y + 1,
        width: max_x + 1,
    }
}

impl Input {
    fn adjacent_cells(&self, x: isize, y: isize) -> Option<Vec<(Coord, Number)>> {
        let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let cells = offsets
                .into_iter()
                .filter_map(|(offset_x, offset_y)| {
                    let new_x = x + offset_x;
                    let new_y = y + offset_y;
                    self.height_at_cell(new_x, new_y)
                        .map(|n| ((new_x, new_y), n))
                })
                .collect();
            Some(cells)
        } else {
            None
        }
    }

    fn height_at_cell(&self, x: isize, y: isize) -> Option<Number> {
        self.grid.get(&(x, y)).copied()
    }

    fn is_low_point(&self, x: isize, y: isize) -> Option<bool> {
        self.height_at_cell(x, y).map(|height| {
            self.adjacent_cells(x, y)
                .unwrap()
                .iter()
                .all(|(_coord, other)| other > &height)
        })
    }

    fn sum_low_points_risk(&self) -> Number {
        self.grid
            .iter()
            .filter_map(|((x, y), n)| {
                if let Some(true) = self.is_low_point(*x, *y) {
                    Some(n + 1)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[aoc(day9, part1)]
fn calculate_risk_sum(input: &Input) -> Number {
    input.sum_low_points_risk()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_low_points_risk() {
        let raw_input = r#"
2199943210
3987894921
9856789892
8767896789
9899965678
"#;
        let input = input_parser(raw_input);
        assert_eq!(input.sum_low_points_risk(), 15);
    }
}
