use color_eyre::Result;

pub fn process_part1(input: &str) -> Result<usize> {
    let problems = parse_input(input)?;
    Ok(problems.iter().map(HomeworkProblem::solve).sum())
}

pub fn process_part2(input: &str) -> Result<usize> {
    Ok(0)
}

#[derive(Clone, PartialEq, Debug)]
enum Op {
    Add,
    Mult,
}

#[derive(Clone, PartialEq, Debug)]
struct HomeworkProblem {
    numbers: Vec<usize>,
    op: Op,
}

impl HomeworkProblem {
    fn solve(&self) -> usize {
        match self.op {
            Op::Add => self.numbers.iter().fold(0, |a, b| a + b),
            Op::Mult => self.numbers.iter().fold(1, |a, b| a * b),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<HomeworkProblem>> {
    let rows: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let ops: Vec<Op> = rows
        .last()
        .unwrap()
        .iter()
        .map(|&s| match s {
            "*" => Op::Mult,
            "+" => Op::Add,
            _ => panic!("oops"),
        })
        .collect();

    Ok(ops
        .into_iter()
        .enumerate()
        .map(|(i, op)| HomeworkProblem {
            op,
            numbers: rows
                .iter()
                .take(rows.len() - 1)
                .map(|row| row[i].parse().unwrap())
                .collect(),
        })
        .collect())
}

#[cfg(test)]
mod tests {

    use crate::{HomeworkProblem, Op};

    use super::{parse_input, process_part1, process_part2};

    const TEST_INPUT: &str = "\
    123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +  ";

    #[test]
    fn test_parse_input() {
        let probs = parse_input(TEST_INPUT).unwrap();

        assert_eq!(
            probs[0],
            HomeworkProblem {
                op: Op::Mult,
                numbers: vec![123, 45, 6]
            }
        );
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(TEST_INPUT).unwrap(), 4277556);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(TEST_INPUT).unwrap(), 14);
    }
}
