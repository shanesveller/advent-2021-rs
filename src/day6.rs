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

#[aoc(day6, part1)]
fn part1(input: &[Lanternfish]) -> usize {
    (1..=80).fold((Vec::new(), input.to_vec()), |(mut new, mut current), _day| {
        for this in current.iter_mut() {
            if let Some(spawn) = this.age() {
                new.push(spawn);
            }
        }

        current.append(&mut new);
        (new, current)
    }).1.len()
}
