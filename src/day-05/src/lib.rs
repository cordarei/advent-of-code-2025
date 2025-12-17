use std::ops::RangeInclusive;

use color_eyre::Result;

use nom::{
    self, IResult, Parser,
    character::complete::{char, digit1},
    combinator::map_res,
};

use thousands::Separable;

pub fn process_part1(input: &str) -> Result<usize> {
    let PuzzleInput {
        fresh_ranges,
        available_ids,
    } = parse_input(input)?;

    let count = available_ids
        .iter()
        .filter(|id| fresh_ranges.iter().any(|range| range.contains(id)))
        .count();

    Ok(count)
}

pub fn process_part2(input: &str) -> Result<usize> {
    let PuzzleInput {
        mut fresh_ranges,
        available_ids: _,
    } = parse_input(input)?;

    fresh_ranges.sort_by_key(|r| (r.start().clone(), r.end().clone()));
    let mut count = 0;
    let mut starting_at = 0;

    for range in fresh_ranges.iter() {
        eprintln!(
            "Range: {range:?} Start: {} Count: {}",
            starting_at.separate_with_underscores(),
            count.separate_with_underscores(),
        );
        if starting_at < *range.start() {
            starting_at = *range.start();
        }
        eprintln!(
            "    Move starting_at to: {}",
            starting_at.separate_with_underscores()
        );

        if starting_at <= *range.end() {
            count += range.end() - starting_at + 1;
            starting_at = range.end() + 1;
        }
        eprintln!(
            "    Update count to: {} next starting at: {}",
            count.separate_with_underscores(),
            starting_at.separate_with_underscores(),
        );
    }

    Ok(count)
}

#[derive(Clone, PartialEq, Debug)]
struct PuzzleInput {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    available_ids: Vec<usize>,
}

fn parse_input(input: &str) -> Result<PuzzleInput> {
    let mut fresh_ranges = vec![];
    let mut available_ids = vec![];

    for line in input.lines() {
        if let Ok((_, r)) = range(line) {
            fresh_ranges.push(r);
        } else if let Ok((_, n)) = number(line) {
            available_ids.push(n);
        }
    }

    Ok(PuzzleInput {
        fresh_ranges,
        available_ids,
    })
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse()).parse(input)
}

fn range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (rest, (first, _, last)) = (number, char('-'), number).parse(input)?;
    Ok((rest, (first)..=(last)))
}

#[cfg(test)]
mod tests {
    use crate::PuzzleInput;

    use super::{parse_input, process_part1, process_part2};

    const TEST_INPUT: &str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(TEST_INPUT).unwrap(),
            PuzzleInput {
                fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
                available_ids: vec![1, 5, 8, 11, 17, 32]
            }
        );
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(TEST_INPUT).unwrap(), 3);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(TEST_INPUT).unwrap(), 14);
    }
}
