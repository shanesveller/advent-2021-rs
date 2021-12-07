use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy, Debug)]
struct Lanternfish(u8);

#[derive(Clone, Debug)]
struct LanternfishSchool([usize; 9]);

impl LanternfishSchool {
    fn new(fish: &[Lanternfish]) -> Self {
        let mut counts = [0usize; 9];
        fish.iter().for_each(|fish| counts[usize::from(fish.0)] += 1);
        Self(counts)
    }

    fn simulate_day(&mut self) {
        let new = self.0[0];

        for idx in 1..=8 {
            self.0[idx - 1] = self.0[idx];
        }

        self.0[6] += new;
        self.0[8] = new;
    }

    fn count(&self) -> usize {
        self.0.iter().sum()
    }
}

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<Lanternfish> {
   input.split(',').map(|n| Lanternfish(str::parse::<u8>(n).unwrap())).collect()
}

#[aoc(day6, part1)]
fn part1(input: &[Lanternfish]) -> usize {
    let mut school = LanternfishSchool::new(input);
    for _day in 1..=80 {
        school.simulate_day();
    }
    school.count()
}

#[aoc(day6, part2)]
fn part2(input: &[Lanternfish]) -> usize {
    let mut school = LanternfishSchool::new(input);
    for _day in 1..=256 {
        school.simulate_day();
    }
    school.count()
}
