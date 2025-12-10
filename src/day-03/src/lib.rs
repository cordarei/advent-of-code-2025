use std::cmp::Ordering;

use color_eyre::Result;

pub fn process_part1(input: &str) -> Result<usize> {
    let banks = parse_input(input);
    Ok(banks.into_iter().map(|b| max_joltage(&b) as usize).sum())
}

pub fn process_part2(input: &str) -> Result<usize> {
    let banks = parse_input(input);
    Ok(banks.into_iter().map(|b| maxer_joltage(&b) as usize).sum())
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .split('\n')
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .filter(|b: &Vec<u8>| b.len() > 0)
        .collect()
}

fn max_joltage(bank: &[u8]) -> u8 {
    let n = bank.len();
    (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .map(|(i, j)| bank[i] * 10 + bank[j])
        .max()
        .unwrap()
}

#[cfg(false)]
mod combinations {

    pub const SUBSET_SIZE: usize = 12;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Combination {
        n: usize,
        indices: [u8; SUBSET_SIZE],
    }

    impl Combination {
        fn new(n: usize) -> Combination {
            assert!(n >= SUBSET_SIZE);

            let mut indices = [0_u8; SUBSET_SIZE];
            for i in 0..SUBSET_SIZE {
                indices[i] = i as u8;
            }
            Combination { n, indices }
        }

        fn inc(&mut self) -> bool {
            let n = self.n;

            if self.indices[0] as usize == n - SUBSET_SIZE {
                return false;
            }

            let mut i = SUBSET_SIZE - 1;
            while self.indices[i] as usize == i + n - SUBSET_SIZE {
                i -= 1;
            }

            self.indices[i] += 1;
            for j in i + 1..SUBSET_SIZE {
                self.indices[j] = self.indices[j - 1] + 1;
            }

            true
        }
    }

    pub struct CombIterator {
        item: Combination,
        first: bool,
    }

    impl CombIterator {
        fn new(start: Combination) -> CombIterator {
            CombIterator {
                item: start,
                first: true,
            }
        }
    }

    impl Iterator for CombIterator {
        type Item = Combination;

        fn next(&mut self) -> Option<Self::Item> {
            if self.first {
                self.first = false;
                Some(self.item.clone())
            } else if self.item.inc() {
                Some(self.item.clone())
            } else {
                None
            }
        }
    }
    pub fn combinations(n: usize) -> CombIterator {
        CombIterator::new(Combination::new(n))
    }

    pub fn bank_value_from_combination(bank: &[u8], comb: &Combination) -> usize {
        comb.indices
            .iter()
            .fold(0 as usize, |val, &i| val * 10 + bank[i as usize] as usize)
    }
}

fn maxer_joltage(bank: &[u8]) -> usize {
    assert!(bank.len() > 0);

    let mut indices = [0; 12];

    let cmp = |&i: &usize, &j: &usize| match bank[i].cmp(&bank[j]) {
        Ordering::Equal => j.cmp(&i),
        o => o,
    };

    // highest digit that has >= 12 (n) digits remaining to its right
    indices[0] = (0..=bank.len() - 12).max_by(cmp).unwrap();

    for i in 1..12 {
        indices[i] = ((indices[i - 1] + 1)..=(bank.len() - 12 + i))
            .max_by(cmp)
            .unwrap();
    }

    indices
        .iter()
        .fold(0 as usize, |val, &i| val * 10 + bank[i] as usize)
}

#[cfg(test)]
mod tests {
    use super::max_joltage;
    use super::maxer_joltage;
    use super::parse_input;

    const TEST_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_max_joltage() {
        assert_eq!(max_joltage(&[1, 1]), 11);

        let banks = parse_input(TEST_INPUT);
        assert_eq!(
            banks
                .iter()
                .map(|b| max_joltage(&b) as usize)
                .sum::<usize>(),
            357
        );
    }

    #[test]
    fn test_maxer_joltage() {
        let banks = parse_input(TEST_INPUT);

        assert_eq!(maxer_joltage(&banks[0]), 987654321111);
        assert_eq!(maxer_joltage(&banks[1]), 811111111119);
        assert_eq!(maxer_joltage(&banks[2]), 434234234278);
        assert_eq!(maxer_joltage(&banks[3]), 888911112111);
        assert_eq!(
            banks
                .iter()
                .map(|b| maxer_joltage(&b) as usize)
                .sum::<usize>(),
            3121910778619
        )
    }
}
