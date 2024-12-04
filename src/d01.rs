use anyhow::{bail, Result};

pub fn part_one(data: &str) -> Result<i64> {
    let mut lft = vec![];
    let mut rgt = vec![];

    for values in data
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(' '))
    {
        match values {
            Some((l, r)) => match (l.trim().parse::<i64>(), r.trim().parse::<i64>()) {
                (Ok(l), Ok(r)) => {
                    lft.push(l);
                    rgt.push(r);
                }
                _ => bail!("could not parse '{}' or '{}'", l, r),
            },
            _ => bail!("could not parse"),
        }
    }

    lft.sort();
    rgt.sort();

    let mut count = 0;
    for (l, r) in lft.into_iter().zip(rgt) {
        count = count + (l - r).abs();
    }
    Ok(count)
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
