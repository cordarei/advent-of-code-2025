use color_eyre::Result;
use nom::{
    self, IResult, Parser,
    branch::alt,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list1,
};

pub fn process_part1(input: &str) -> Result<i32> {
    let turns = parse_input(input)?;
    Ok(count_zeros(&turns))
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Turn {
    Left(u32),
    Right(u32),
}

pub fn parse_turn(input: &str) -> IResult<&str, Turn> {
    let (rest, direction_char) = alt((char('L'), char('R'))).parse(input)?;
    let (rest, val) = map_res(digit1, |result: &str| result.parse::<u32>()).parse(rest)?;

    let turn = match direction_char {
        'L' => Turn::Left(val),
        'R' => Turn::Right(val),
        _ => panic!("oops"),
    };

    Ok((rest, turn))
}

pub fn parse_input(input: &str) -> Result<Vec<Turn>> {
    let (_rest, turns) = separated_list1(char('\n'), parse_turn)
        .parse(input)
        .expect("problem parsing full input");

    Ok(turns)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Dial {
    pub location: u8,
}

impl Dial {
    pub fn new() -> Dial {
        Dial { location: 50 }
    }
    pub fn with_location(location: u8) -> Dial {
        Dial { location }
    }

    pub fn turn(&self, t: Turn) -> Self {
        let new_loc = match t {
            Turn::Left(val) => self.location as i32 - (val % 100) as i32,
            Turn::Right(val) => self.location as i32 + (val % 100) as i32,
        };
        let new_loc = if new_loc < 0 { new_loc + 100 } else { new_loc };
        let new_loc = new_loc % 100;
        Self {
            location: new_loc as u8,
        }
    }
}

pub fn count_zeros(turns: &Vec<Turn>) -> i32 {
    let (_, n) = turns.iter().fold((Dial::new(), 0), |(d, n), &t| {
        let d = d.turn(t);
        let n = if d.location == 0 { n + 1 } else { n };
        (d, n)
    });
    n
}

#[cfg(test)]
mod tests {
    use crate::count_zeros;

    use super::Dial;
    use super::Turn;
    use super::parse_input;

    #[test]
    fn test_parsing() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(
            parse_input(input).unwrap(),
            vec![
                Turn::Left(68),
                Turn::Left(30),
                Turn::Right(48),
                Turn::Left(5),
                Turn::Right(60),
                Turn::Left(55),
                Turn::Left(1),
                Turn::Left(99),
                Turn::Right(14),
                Turn::Left(82),
            ]
        );
        assert_eq!(parse_input("R1234").unwrap(), vec![Turn::Right(1234)])
    }

    #[test]
    fn test_dial() {
        let d = Dial::new();
        assert_eq!(d.turn(Turn::Left(1)), Dial::with_location(49));
        assert_eq!(d.turn(Turn::Right(1)), Dial::with_location(51));
        assert_eq!(d.turn(Turn::Right(49)), Dial::with_location(99));
        assert_eq!(d.turn(Turn::Left(51)), Dial::with_location(99));
        assert_eq!(d.turn(Turn::Right(900)), Dial::with_location(50));

        assert_eq!(d.turn(Turn::Right(50)), Dial::with_location(0));
        assert_eq!(d.turn(Turn::Left(50)), Dial::with_location(0));
        assert_eq!(
            Dial::with_location(99).turn(Turn::Right(99)),
            Dial::with_location(98)
        );
        assert_eq!(
            Dial::with_location(1).turn(Turn::Left(99)),
            Dial::with_location(2)
        );
    }

    #[test]
    fn test_count_zeros() {
        assert_eq!(count_zeros(&vec![Turn::Left(1)]), 0);
        assert_eq!(count_zeros(&vec![Turn::Left(50)]), 1);
        assert_eq!(count_zeros(&vec![Turn::Right(50)]), 1);

        let turns = parse_input(
            "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82",
        )
        .unwrap();

        assert_eq!(count_zeros(&turns), 3);
    }
}
