use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(str::parse::<u32>)
        .collect::<Result<Vec<u32>, _>>()
        .unwrap()
}

#[aoc(day1, part1)]
pub fn windows(input: &[u32]) -> u32 {
    input.windows(2).fold(0, |sum, pair| match pair {
        &[prev, next] => {
            if next > prev {
                sum + 1
            } else {
                sum
            }
        }
        _ => unreachable!(),
    })
}
