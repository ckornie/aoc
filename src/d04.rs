use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Part {
    number: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/04.example");
        assert_eq!(1, 1);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/04.actual");
        assert_eq!(1, 1);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/04.example");
        assert_eq!(1, 1);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/04.actual");
        assert_eq!(1, 1);
        Ok(())
    }
}
