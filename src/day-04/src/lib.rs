use std::cmp::min;

use color_eyre::Result;

pub fn process_part1(input: &str) -> Result<usize> {
    let rolls: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();
    Ok(0)
}

pub fn process_part2(_input: &str) -> Result<usize> {
    Ok(0)
}

fn _parse_input(_input: &str) -> Vec<Vec<u8>> {
    vec![]
}

fn neighbors(row: usize, col: usize, rolls: &Vec<Vec<bool>>) -> i32 {
    assert!(row < rolls.len());
    assert!(col < rolls[row].len());

    let mut count = 0;
    for r in row.saturating_sub(1)..=min(row + 1, rolls.len() - 1) {
        for c in col.saturating_sub(1)..=min(col + 1, rolls[row].len() - 1) {
            if rolls[r][c] && (r != row || c != col) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::neighbors;

    const TEST_INPUT: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_neighbors() {
        let rolls: Vec<Vec<bool>> = TEST_INPUT
            .lines()
            .map(|l| l.chars().map(|c| c == '@').collect())
            .collect();

        assert_eq!(neighbors(0, 0, &rolls), 2);
    }
}
