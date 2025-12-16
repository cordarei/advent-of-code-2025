use color_eyre::Result;

pub fn process_part1(_input: &str) -> Result<usize> {
    Ok(0)
}

pub fn process_part2(_input: &str) -> Result<usize> {
    Ok(0)
}

fn _parse_input(_input: &str) -> Vec<Vec<u8>> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::{process_part1, process_part2};

    const TEST_INPUT: &str = "\
";

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(TEST_INPUT).unwrap(), 13);
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(TEST_INPUT).unwrap(), 43);
    }
}
