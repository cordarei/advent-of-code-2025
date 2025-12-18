use color_eyre::Result;

pub fn process_part1(input: &str) -> Result<usize> {
    let problems = parse_input(input)?;
    Ok(problems.iter().map(HomeworkProblem::solve).sum())
}

pub fn process_part2(input: &str) -> Result<usize> {
    let problems = parse_input_part2(input)?;
    Ok(problems.iter().map(HomeworkProblem::solve).sum())
}

#[derive(Clone, PartialEq, Debug, Copy)]
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

fn parse_input_part2(input: &str) -> Result<Vec<HomeworkProblem>> {
    let rows: Vec<&str> = input.lines().collect();
    let row_length = rows[0].len();
    let nrows = rows.len();

    let ops: Vec<Op> = rows[nrows - 1]
        .split_ascii_whitespace()
        .map(|s| match s {
            "*" => Op::Mult,
            "+" => Op::Add,
            _ => panic!("oops"),
        })
        .collect();

    let rows: Vec<Vec<_>> = rows
        .iter()
        .take(nrows - 1)
        .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect();
    let mut problems: Vec<HomeworkProblem> = Vec::new();
    let mut numbers: Vec<usize> = Vec::new();
    for i in (0..row_length).rev() {
        let mut number: Option<usize> = None;
        for j in 0..nrows - 1 {
            match (number, rows[j][i]) {
                (None, Some(digit)) => {
                    number = Some(digit as usize);
                }
                (Some(val), Some(digit)) => {
                    number = Some(val * 10 + digit as usize);
                }
                (_, None) => {}
            }
        }

        if let Some(number) = number {
            numbers.push(number);
        } else {
            let op_index = ops.len() - problems.len() - 1;
            let op = ops[op_index];
            problems.push(HomeworkProblem { numbers, op });
            numbers = Vec::new();
        }
    }
    if !numbers.is_empty() {
        let op_index = ops.len() - problems.len() - 1;
        let op = ops[op_index];
        problems.push(HomeworkProblem { numbers, op });
    }

    problems.reverse();
    Ok(problems)
}

#[cfg(test)]
mod tests {

    use crate::{HomeworkProblem, Op};

    use super::{parse_input, parse_input_part2, process_part1, process_part2};

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
    fn test_parse_input_part2() {
        let mut input = String::new();
        input.push_str("123 328  51 64 \n");
        input.push_str(" 45 64  387 23 \n");
        input.push_str("  6 98  215 314\n");
        input.push_str("*   +   *   +  \n");

        let probs = parse_input_part2(&input).unwrap();

        assert_eq!(
            probs[0],
            HomeworkProblem {
                op: Op::Mult,
                numbers: vec![356, 24, 1]
            }
        );

        assert_eq!(
            probs[1],
            HomeworkProblem {
                op: Op::Add,
                numbers: vec![8, 248, 369]
            }
        );

        assert_eq!(
            probs[2],
            HomeworkProblem {
                op: Op::Mult,
                numbers: vec![175, 581, 32]
            }
        );

        assert_eq!(
            probs[3],
            HomeworkProblem {
                op: Op::Add,
                numbers: vec![4, 431, 623]
            }
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            HomeworkProblem {
                op: Op::Mult,
                numbers: vec![175, 581, 32]
            }
            .solve(),
            3253600
        )
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(TEST_INPUT).unwrap(), 4277556);
    }

    #[test]
    fn test_process_part2() {
        let mut input = String::new();
        input.push_str("123 328  51 64 \n");
        input.push_str(" 45 64  387 23 \n");
        input.push_str("  6 98  215 314\n");
        input.push_str("*   +   *   +  \n");

        assert_eq!(process_part2(&input).unwrap(), 3263827);
    }
}
