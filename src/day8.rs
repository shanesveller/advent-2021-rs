use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};
use std::collections::HashMap;

#[cfg(test)]
type Number = u8;

#[cfg(test)]
const SIGNAL_COUNT: [Number; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

#[derive(Debug, PartialEq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

fn infer_digit(sequence: &[Segment], known: &HashMap<u8, Digit>) -> Option<Digit> {
    known.get(&u8::try_from(sequence.len()).unwrap()).copied()
}

fn segment_from_char(c: char) -> Segment {
    match c {
        'a' => Segment::A,
        'b' => Segment::B,
        'c' => Segment::C,
        'd' => Segment::D,
        'e' => Segment::E,
        'f' => Segment::F,
        'g' => Segment::G,
        _ => unreachable!(),
    }
}

type Sequence = Vec<Segment>;

fn parse_segment(input: &str) -> IResult<&str, Segment> {
    map(one_of("abcdefg"), segment_from_char)(input)
}

fn parse_sequence(input: &str) -> IResult<&str, Vec<Sequence>> {
    separated_list1(tag(" "), many1(parse_segment))(input)
}

fn parse_inputs_and_outputs(input: &str) -> IResult<&str, (Vec<Sequence>, Vec<Sequence>)> {
    separated_pair(parse_sequence, tag(" | "), parse_sequence)(input)
}

#[allow(clippy::type_complexity)]
fn parse_all_inputs(input: &str) -> IResult<&str, Vec<(Vec<Sequence>, Vec<Sequence>)>> {
    separated_list1(line_ending, parse_inputs_and_outputs)(input)
}

#[aoc_generator(day8)]
fn input_parser(input: &str) -> Vec<(Vec<Sequence>, Vec<Sequence>)> {
    if let Ok(("", pairs)) = parse_all_inputs(input) {
        pairs
    } else {
        panic!()
    }
}

#[aoc(day8, part1)]
fn part1(input: &[(Vec<Sequence>, Vec<Sequence>)]) -> usize {
    let known = [
        (2, Digit::One),
        (4, Digit::Four),
        (3, Digit::Seven),
        (7, Digit::Eight),
    ]
    .into_iter()
    .collect();
    input
        .iter()
        .flat_map(|(_left, right)| right)
        .filter_map(|seq| infer_digit(&seq[..], &known))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_digit() {
        let candidates = "abcdefgh";
        let known = [
            (2, Digit::One),
            (4, Digit::Four),
            (3, Digit::Seven),
            (7, Digit::Eight),
        ]
        .into_iter()
        .collect();

        for n in [1, 4, 7, 8] {
            let count = SIGNAL_COUNT[n];
            let raw_sequence = &candidates[0..(count as usize)];

            if let Ok(("", sequence)) = parse_sequence(raw_sequence) {
                assert_eq!(
                    infer_digit(&sequence[0], &known),
                    Some(*known.get(&count).unwrap())
                );
            } else {
                panic!()
            }
        }
    }
}
