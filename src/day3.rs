use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> (u8, Vec<u16>) {
    let inputs = input
        .lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect();
    let width = input.lines().next().unwrap().len();
    (width.try_into().unwrap(), inputs)
}

fn count_digits(inputs: &[u16], width: u8) -> Vec<(u16, u16)> {
    (0..width)
        .map(|column| count_for_column(inputs, column))
        .collect()
}

fn count_for_column(inputs: &[u16], column: u8) -> (u16, u16) {
    inputs.iter().fold((0, 0), |mut counts, n| {
        if (n & (1u16 << column)) > 0 {
            counts.1 += 1
        } else {
            counts.0 += 1
        }
        counts
    })
}

#[aoc(day3, part1)]
fn part_1((width, inputs): &(u8, Vec<u16>)) -> u32 {
    let counts = count_digits(inputs, *width);

    let mut gamma = 0;
    let mut epsilon = 0;

    counts.iter().enumerate().for_each(|(pos, (zeros, ones))| {
        if ones > zeros {
            gamma += 1u32 << pos;
        } else {
            epsilon += 1u32 << pos;
        }
    });
    gamma * epsilon
}

#[aoc(day3, part2)]
fn part_2((width, inputs): &(u8, Vec<u16>)) -> u32 {
    let oxygen = filter_by_common(inputs, *width, Ordering::Greater, 1);
    let carbon = filter_by_common(inputs, *width, Ordering::Less, 0);
    (oxygen as u32) * (carbon as u32)
}

fn filter_by_common(inputs: &[u16], width: u8, keep: Ordering, default: u16) -> u16 {
    *(0..width)
        .rev()
        .fold(inputs.to_vec(), |mut candidates, column| {
            if candidates.len() > 1 {
                let (zeros, ones) = count_for_column(&candidates, column);

                let comparison = zeros.cmp(&ones);

                use Ordering::*;

                match (keep, comparison, default) {
                    (Greater, Greater, _n) => {
                        keep_zeros(&mut candidates, column);
                    }
                    (Equal, Equal, 0) => {
                        keep_zeros(&mut candidates, column);
                    }
                    (Less, Less, _n) => {
                        keep_zeros(&mut candidates, column);
                    }
                    (Less, Equal, 0) => {
                        keep_zeros(&mut candidates, column);
                    }
                    _ => {
                        keep_ones(&mut candidates, column);
                    }
                }
            }
            candidates
        })
        .first()
        .unwrap()
}

fn keep_zeros(inputs: &mut Vec<u16>, column: u8) {
    inputs.retain(|&n| n & 1u16 << column == 0)
}
fn keep_ones(inputs: &mut Vec<u16>, column: u8) {
    inputs.retain(|&n| (n & 1u16 << column) > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#
        .trim_start();

        let (width, inputs) = parse_input(input);

        assert_eq!(part_2(&(width, inputs)), 230);
    }

    #[test]
    fn test_position() {
        fn has_zero_pos(n: u8, position: u8) -> bool {
            n & 1u8 << position == 0
        }
        fn has_one_pos(n: u8, position: u8) -> bool {
            n & 1u8 << position > 0
        }

        assert!(has_one_pos(0b0010, 1));
        assert!(has_one_pos(0b0010, 1));
        assert!(has_zero_pos(0b0010, 0));
        assert!(has_zero_pos(0b0010, 2));
        assert!(has_zero_pos(0b0010, 3));
        assert!(has_one_pos(0b10010, 4));
        assert!(has_one_pos(0b110010, 5));
        assert!(has_zero_pos(0b0110010, 0));
        assert!(has_zero_pos(0b0110010, 2));
        assert!(has_zero_pos(0b0110010, 3));
        assert!(has_zero_pos(0b0110010, 6));
    }
}
