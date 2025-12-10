use std::ops::RangeInclusive;

use color_eyre::Result;
use nom::{
    self, IResult, Parser,
    branch::alt,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
};

pub fn process_part1(input: &str) -> Result<u32> {
    let _ = parse_input(input);
    Ok(0)
}

pub fn process_part2(input: &str) -> Result<u32> {
    Ok(0)
}

// --------
// INPUT
// --------

fn parse_input(input: &str) -> Result<Vec<RangeInclusive<usize>>> {
    let (_rest, ranges) = separated_list1(char(','), range)
        .parse(input)
        .expect("error parsing input");

    Ok(ranges)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse()).parse(input)
}

fn range(input: &str) -> IResult<&str, RangeInclusive<usize>> {
    let (rest, (first, _, last)) = (number, char('-'), number).parse(input)?;
    Ok((rest, (first)..=(last)))
}

// --------
// PART 1
// --------

fn is_invalid_id(id: usize) -> bool {
    let digits: Vec<u8> = id
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8).unwrap())
        .collect();

    let n = digits.len();
    let mid = n / 2;

    if n % 2 == 0 {
        digits[0..mid] == digits[mid..n]
    } else {
        false
    }
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_parse_input() {
        assert_eq!(
            super::parse_input("11-22,95-115,998-1012,1188511880-1188511890").unwrap(),
            [11..=22, 95..=115, 998..=1012, 1188511880..=1188511890]
        )
    }

    #[test]
    fn test_is_invalid_id() {
        use super::is_invalid_id;

        assert_eq!(is_invalid_id(11), true);
        assert_eq!(is_invalid_id(12), false);
        assert_eq!(is_invalid_id(1188511885), true);
    }
}
