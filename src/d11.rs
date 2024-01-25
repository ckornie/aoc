use anyhow::Result;

pub fn part_one(_input: &str) -> Result<i64> {
    Ok(0)
}

pub fn part_two(_input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/11.example");
        assert_eq!(part_one(input)?, 0);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/11.actual");
        assert_eq!(part_one(input)?, 0);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/11.example");
        assert_eq!(part_two(input)?, 0);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/11.actual");
        assert_eq!(part_two(input)?, 0);
        Ok(())
    }
}
