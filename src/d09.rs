use anyhow::{Context, Result};
use itertools::Itertools;

fn readings(input: &str) -> Result<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i64>())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
        .with_context(|| "unexpected format")
}

fn predict(readings: &Vec<i64>) -> i64 {
    if readings.iter().all_equal() {
        return readings.last().map(|&e| e).unwrap_or_default();
    }

    let next = predict(
        &readings
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| b - a)
            .collect(),
    );

    readings.last().map(|&e| e + next).unwrap_or_default()
}

fn extrapolate(readings: &Vec<i64>) -> i64 {
    if readings.iter().all(|&reading| reading == 0) {
        return readings.first().map(|&e| e).unwrap_or_default();
    }

    let next = extrapolate(
        &readings
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| b - a)
            .collect(),
    );

    readings.first().map(|&e| e - next).unwrap_or_default()
}

pub fn part_one(input: &str) -> Result<i64> {
    Ok(readings(input)?
        .iter()
        .map(|readings| predict(readings))
        .sum())
}

pub fn part_two(input: &str) -> Result<i64> {
    Ok(readings(input)?
        .iter()
        .map(|readings| extrapolate(readings))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/09.example");
        assert_eq!(part_one(input)?, 114);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/09.actual");
        assert_eq!(part_one(input)?, 1_708_206_096);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/09.example");
        assert_eq!(part_two(input)?, 2);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/09.actual");
        assert_eq!(part_two(input)?, 1050);
        Ok(())
    }
}
