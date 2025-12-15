use std::cmp::min;

use color_eyre::Result;

pub fn process_part1(input: &str) -> Result<usize> {
    let rolls: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();
    let nrows = rolls.len();
    let ncols = rolls[0].len();

    let count = (0..nrows)
        .flat_map(|i| (0..ncols).map(move |j| (i, j)))
        .filter(|&(i, j)| rolls[i][j] && neighbors(i, j, &rolls) < 4)
        .count();

    Ok(count)
}

pub fn process_part2(input: &str) -> Result<usize> {
    let mut rolls: Vec<Vec<bool>> = input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect();
    let nrows = rolls.len();
    let ncols = rolls[0].len();

    let mut count = 0;

    loop {
        let available: Vec<(usize, usize)> = (0..nrows)
            .flat_map(|i| (0..ncols).map(move |j| (i, j)))
            .filter(|&(i, j)| rolls[i][j] && neighbors(i, j, &rolls) < 4)
            .collect();

        count += available.len();

        for &(i, j) in available.iter() {
            rolls[i][j] = false;
        }

        if available.len() == 0 {
            break;
        }
    }

    Ok(count)
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
    use super::{neighbors, process_part1, process_part2};

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
        assert_eq!(neighbors(0, 1, &rolls), 4);
        assert_eq!(neighbors(0, 2, &rolls), 3);
        assert_eq!(neighbors(0, 7, &rolls), 4);
        assert_eq!(neighbors(4, 4, &rolls), 8);

        assert_eq!(neighbors(0, 9, &rolls), 3);
        assert_eq!(neighbors(9, 0, &rolls), 1);
        assert_eq!(neighbors(9, 9, &rolls), 2);
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(TEST_INPUT).unwrap(), 43);
    }
}
