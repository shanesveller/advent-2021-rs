use aoc_runner_derive::{aoc, aoc_generator};

type Number = usize;

struct Input {
    numbers: Vec<Number>,
    row_count: isize,
    width: isize,
}

#[aoc_generator(day9)]
fn input_parser(input: &str) -> Input {
    let mut width = 0;
    let numbers: Vec<Number> = input
        .lines()
        .flat_map(|l| {
            width = l.len();
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();
    let row_count: isize = numbers.len().try_into().unwrap();

    Input {
        numbers,
        row_count,
        width: width.try_into().unwrap(),
    }
}

impl Input {
    fn adjacent_heights(&self, index: usize) -> Vec<Number> {
        let offsets: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let (x, y) = self.coordinate_for_index(index);
        offsets
            .into_iter()
            .filter_map(|(offset_x, offset_y)| {
                let new_x = x + offset_x;
                let new_y = y + offset_y;
                if new_x >= 0 && new_x <= self.width && new_y >= 0 && new_y <= self.row_count {
                    Some(self.index_for_coordinate(new_x, new_y).unwrap())
                } else {
                    None
                }
            })
            .filter_map(|idx| self.numbers.get::<usize>(idx))
            .copied()
            .collect()
    }

    fn coordinate_for_index(&self, index: usize) -> (isize, isize) {
        (
            index
                .rem_euclid(self.width.try_into().unwrap())
                .try_into()
                .unwrap(),
            index
                .div_euclid(self.width.try_into().unwrap())
                .try_into()
                .unwrap(),
        )
    }

    fn index_for_coordinate(&self, x: isize, y: isize) -> Option<usize> {
        Some((y * self.width + x).try_into().unwrap())
    }

    fn is_low_point(&self, index: usize) -> bool {
        let height = self.numbers[index];
        self.adjacent_heights(index)
            .iter()
            .all(|other| other > &height)
    }

    fn sum_low_points_risk(&self) -> Number {
        self.numbers
            .iter()
            .enumerate()
            .filter_map(|(idx, n)| {
                if self.is_low_point(idx) {
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
