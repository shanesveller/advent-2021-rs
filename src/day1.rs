use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(str::parse::<usize>)
        .collect::<Result<Vec<usize>, _>>()
        .unwrap()
}

#[aoc(day1, part1)]
pub fn windows(input: &[usize]) -> usize {
    input.windows(2).fold(0, |sum, pair| match *pair {
        [prev, next] => {
            if next > prev {
                sum + 1
            } else {
                sum
            }
        }
        _ => unreachable!(),
    })
}

#[aoc(day1, part2)]
pub fn nested_windows(input: &[usize]) -> usize {
    let triads: Vec<&[usize]> = input.windows(3).collect();
    triads.windows(2).fold(0, |sum, pair| match *pair {
        [prev, next] => {
            if prev.iter().sum::<usize>() < next.iter().sum::<usize>() {
                sum + 1
            } else {
                sum
            }
        }
        _ => unreachable!(),
    })
}
