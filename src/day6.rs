use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
struct Lanternfish(u8);

impl Lanternfish {
    fn age(&mut self) -> Option<Lanternfish> {
        match self.0 {
            0 => {
                self.0 = 6;
                Some(Lanternfish(8))
            }
            1..=8 => {
                self.0 -= 1;
                None
            }
            _ => unreachable!()
        }
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Lanternfish> {
   input.split(',').map(|n| Lanternfish(str::parse::<u8>(n).unwrap())).collect()
}

fn simulate_for_days(mut fish: Vec<Lanternfish>, days: usize) -> Vec<Lanternfish> {
    assert!(days >= 1);
    let mut new = Vec::with_capacity(fish.len());
    (1..=days).for_each(|day| {
        println!("Simulating day {}", day);
        for this in fish.iter_mut() {
            if let Some(spawn) = this.age() {
                new.push(spawn);
            }
        }

        fish.append(&mut new);
    });
    fish
}

#[aoc(day6, part1)]
fn part1(input: &[Lanternfish]) -> usize {
    let fish = input.to_vec();
    let total = simulate_for_days(fish, 80);
    total.len()
}

#[aoc(day6, part2)]
fn part2(input: &[Lanternfish]) -> usize {
    let fish = input.to_vec();
    let total = simulate_for_days(fish, 256);
    total.len()
}
