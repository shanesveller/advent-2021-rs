use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    // bytes::complete::tag,
    combinator::{map, recognize},
    error::{context, VerboseError, VerboseErrorKind},
    multi::many0,
    sequence::delimited,
    IResult,
};
use nom_supreme::error::{ErrorTree, Expectation};
use nom_supreme::tag::complete::tag;
use nom_supreme::tag::TagError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Delimiter {
    Paren,
    SquareBracket,
    CurlyBracket,
    AngleBracket,
}

#[derive(Debug, PartialEq)]
struct Chunk {
    delimiter: Delimiter,
    inner: Vec<Chunk>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("invalid delimiter character")]
    InvalidCharacter,
}

impl FromStr for Delimiter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            '(' | ')' => Ok(Self::Paren),
            '[' | ']' => Ok(Self::SquareBracket),
            '{' | '}' => Ok(Self::CurlyBracket),
            '<' | '>' => Ok(Self::AngleBracket),
            _ => Err(Error::InvalidCharacter),
        }
    }
}

impl Delimiter {
    fn score(&self) -> usize {
        use Delimiter::*;
        match self {
            Paren => 3,
            SquareBracket => 57,
            CurlyBracket => 1197,
            AngleBracket => 25137,
        }
    }
}

// type ParseResult<'a> = IResult<&'a str, Chunk, ErrorTree<&'a str>>;
type ParseResult<'a> = IResult<&'a str, Chunk>;
// fn paren_pair(input: &str) -> IResult<&str, Chunk>

macro_rules! chunk_type {
    ($name:ident, $left:literal, $right:literal, $del:path) => {
        fn $name(input: &str) -> ParseResult {
            delimited(
                tag($left),
                map(many0(chunk), |inner| Chunk {
                    delimiter: $del,
                    inner,
                }),
                tag($right),
            )(input)
        }
    };
}

chunk_type!(paren_chunk, "(", ")", Delimiter::Paren);
chunk_type!(square_chunk, "[", "]", Delimiter::SquareBracket);
chunk_type!(curly_chunk, "{", "}", Delimiter::CurlyBracket);
chunk_type!(angle_chunk, "<", ">", Delimiter::AngleBracket);

fn chunk(input: &str) -> ParseResult {
    alt((paren_chunk, square_chunk, curly_chunk, angle_chunk))(input)
}

#[aoc_generator(day10)]
fn input_parser(input: &str) -> Vec<Result<Chunk, String>> {
    input
        .lines()
        .inspect(|l| {
            dbg!(l);
        })
        .map(|l| -> Result<Chunk, String> {
            match chunk(l) {
                Ok((_leftover, chunk)) => Ok(chunk),
                Err(nom::Err::Error(nom::error::Error { input, code })) => {
                    dbg!((&input, &code));
                    Err(")".to_string())
                }
                // Err(nom::Err::Error(nom_supreme::error::ErrorTree::Base { kind, location })) => {
                //     dbg!((&kind, &location));
                //     Err(")".to_string())
                // }
                // Err(nom::Err::Error(nom::error::VerboseError { errors })) => {
                //     dbg!(&errors);
                //     let err = errors.iter().find_map(|e| match e {
                //         (_input, VerboseErrorKind::Context(s)) => Some(s),
                //         _ => None,
                //     });
                //     Err(err.unwrap().to_string())
                // }
                Err(_) => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn sum_illegal_characters(input: &[Result<Chunk, String>]) -> usize {
    input
        .iter()
        .inspect(|res| {
            dbg!(&res);
        })
        .filter(|res| res.is_err())
        .map(|err| Delimiter::from_str(err.as_ref().unwrap_err()))
        .inspect(|del_res| {
            dbg!(&del_res);
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .iter()
        .map(Delimiter::score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk() {
        assert_eq!(
            chunk("()"),
            Ok((
                "",
                Chunk {
                    delimiter: Delimiter::Paren,
                    inner: vec![]
                }
            ))
        );

        assert_eq!(
            chunk("([{<>}])"),
            Ok((
                "",
                Chunk {
                    delimiter: Delimiter::Paren,
                    inner: vec![Chunk {
                        delimiter: Delimiter::SquareBracket,
                        inner: vec![Chunk {
                            delimiter: Delimiter::CurlyBracket,
                            inner: vec![Chunk {
                                delimiter: Delimiter::AngleBracket,
                                inner: vec![]
                            }]
                        }]
                    }]
                }
            ))
        );
    }

    #[test]
    fn test_part1() {
        let raw_input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        let input = input_parser(raw_input);

        assert_eq!(sum_illegal_characters(&input[..]), 26397);
    }
}
