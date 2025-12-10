use std::ops::RangeInclusive;
use std::{iter::Filter, slice::Windows};

use color_eyre::Result;
use nom::{
    self, IResult, Parser,
    branch::alt,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
};

pub fn process_part1(input: &str) -> Result<usize> {
    let ranges = parse_input(input)?;
    let answer = sum_invalid_ids(&ranges);

    Ok(answer)
}

pub fn process_part2(input: &str) -> Result<usize> {
    let ranges = parse_input(input)?;
    let answer = ranges
        .iter()
        .map(|r| r.clone().filter(|&n| is_invalid_id_part2(n)).sum::<usize>())
        .sum();

    Ok(answer)
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

fn is_invalid_id_part2(id: usize) -> bool {
    let digits: Vec<u8> = id
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).map(|d| d as u8).unwrap())
        .collect();

    let n = digits.len();
    let mid = n / 2;

    for m in 1..=mid {
        if n % m == 0 {
            let mut segments = digits.chunks_exact(m);
            if let Some(first) = segments.next() {
                if segments.all(|e| first == e) {
                    return true;
                }
            }
        }
    }

    false
}

fn sum_invalid_ids(ranges: &Vec<RangeInclusive<usize>>) -> usize {
    ranges
        .iter()
        .map(|r| r.clone().filter(|&n| is_invalid_id(n)).sum::<usize>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{parse_input, sum_invalid_ids};

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

    #[test]
    fn test_sum_invalid_ids() {
        assert_eq!(
            sum_invalid_ids(&parse_input(TEST_INPUT).unwrap()),
            1227775554
        )
    }

    #[test]
    fn test_is_invalid_id_part2() {
        use super::is_invalid_id_part2;

        assert_eq!(is_invalid_id_part2(11), true);
        assert_eq!(is_invalid_id_part2(12), false);
        assert_eq!(is_invalid_id_part2(1188511885), true);

        assert_eq!(is_invalid_id_part2(111), true);
        assert_eq!(is_invalid_id_part2(999), true);

        assert_eq!(is_invalid_id_part2(565656), true);
        assert_eq!(is_invalid_id_part2(56565656), true);
        assert_eq!(is_invalid_id_part2(824824824), true);
    }
}
