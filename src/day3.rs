use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect()
}

#[aoc(day3, part1)]
fn part_1(inputs: &[u16]) -> u32 {
    let width = 12;
    let counts: Vec<(u16, u16)> = (0..width)
        .map(|column| {
            inputs.iter().fold((0, 0), |mut counts, n| {
                if (n & (1u16 << column)) > 0 {
                    counts.1 += 1
                } else {
                    counts.0 += 1
                }
                counts
            })
        })
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;

    counts.iter().enumerate().for_each(|(pos, (zeros, ones))| {
        if ones > zeros {
            gamma += 1u32 << pos;
        } else {
            epsilon += 1u32 << pos;
        }
    });
    println!(
        "{} {:012b} {} {:012b} {} {:012b}",
        gamma,
        gamma,
        epsilon,
        epsilon,
        gamma | epsilon,
        gamma | epsilon
    );

    gamma * epsilon
}
