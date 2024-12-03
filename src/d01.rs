use anyhow::{anyhow, Result};
use itertools::Itertools;

fn location(data: &str, idx: usize) -> Result<Vec<i64>, String> {
    data.split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.split(' ')
                .filter(|loc| !loc.is_empty())
                .nth(idx)
                .and_then(|loc| loc.trim().parse::<i64>().ok())
                .ok_or(format!("could not parse {} from {}", idx, row))
        })
        .sorted()
        .collect()
}

pub fn part_one(data: &str) -> Result<i64> {
    match (location(data, 0), location(data, 1)) {
        (Ok(left), Ok(right)) => {
            let mut count = 0;
            for (l, r) in left.into_iter().zip(right) {
                count = count + (l - r).abs();
            }
            Ok(count)
        }
        (Err(msg), _) => Err(anyhow!(msg)),
        (_, Err(msg)) => Err(anyhow!(msg)),
    }
}

pub fn part_two(_data: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = concat!("3   4\n", "4   3\n", "2   5\n", "1   3\n", "3   9\n", "3   3\n",);
        assert_eq!(part_one(input)?, 11);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/01");
        assert_eq!(part_one(input)?, 1_530_215);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = concat!("3   4\n", "4   3\n", "2   5\n", "1   3\n", "3   9\n", "3   3\n",);
        assert_eq!(part_two(input), 0);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/01");
        assert_eq!(part_two(input), 0);
        Ok(())
    }
}
