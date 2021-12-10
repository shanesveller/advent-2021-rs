use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, MinMaxResult};

type Number = u16;

#[aoc_generator(day7)]
fn input_parser(input: &str) -> Vec<Number> {
    input
        .split(',')
        .map(str::parse::<Number>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day7, part1)]
fn part1(input: &[Number]) -> u32 {
    if let MinMaxResult::MinMax(min, max) = input.iter().minmax() {
        let mut lowest_cost: u32 = u32::MAX;
        let mut cost: u32;
        for n in *min..=*max {
            cost = input
                .iter()
                .map(|this| u32::try_from((n as i32 - *this as i32).abs()).unwrap())
                .sum();
            match cost.cmp(&lowest_cost) {
                Ordering::Less => {
                    lowest_cost = cost;
                }
                _ => {}
            }
        }
        lowest_cost
    } else {
        unreachable!();
    }
}

#[aoc(day7, part2)]
fn part2(input: &[Number]) -> u32 {
    if let MinMaxResult::MinMax(min, max) = input.iter().minmax() {
        let mut lowest_cost: u32 = u32::MAX;
        let mut cost: u32;

        let cost_for_distance: Vec<usize> = (*min..=*max)
            .into_iter()
            .scan(0usize, |state, n| {
                *state += n as usize;
                Some(*state)
            })
            .collect();

        for n in *min..=*max {
            cost = input
                .iter()
                .map(|this| u32::try_from((n as i32 - *this as i32).abs()).unwrap())
                .map(|n| cost_for_distance[n as usize] as u32)
                .sum();
            if cost.cmp(&lowest_cost) == Ordering::Less {
                lowest_cost = cost;
            }
        }
        lowest_cost
    } else {
        unreachable!();
    }
}
