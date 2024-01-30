use anyhow::{Context, Result};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Spring {
    Unknown,
    Damaged,
    Fine,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '?' => Spring::Unknown,
            '#' => Spring::Damaged,
            _ => Spring::Fine,
        }
    }
}

impl Spring {
    fn damaged(&self) -> bool {
        match self {
            Self::Damaged => true,
            Self::Unknown => true,
            _ => false,
        }
    }

    fn fine(&self) -> bool {
        match self {
            Self::Fine => true,
            Self::Unknown => true,
            _ => false,
        }
    }
}

fn possibilities(runs: &mut VecDeque<usize>, springs: &[Spring]) -> i64 {
    let mut count = 0;

    if let Some(run) = runs.pop_front() {
        if runs.is_empty() {
            for i in 0..springs.len().saturating_sub(run - 1) {
                count = count
                    + if valid_termination(&springs[i..], run) {
                        1
                    } else {
                        0
                    };

                if springs[i] == Spring::Damaged {
                    runs.push_front(run);
                    return count;
                }
            }
        } else {
            for i in 0..springs.len().saturating_sub(run + 1) {
                let end = i + run;
                count = count
                    + if valid_run(&springs[i..end + 1]) {
                        possibilities(runs, &springs[end + 1..springs.len()])
                    } else {
                        0
                    };

                if springs[i] == Spring::Damaged {
                    runs.push_front(run);
                    return count;
                }
            }
        }
        runs.push_front(run);
    }

    count
}

fn valid_termination(springs: &[Spring], end: usize) -> bool {
    springs[..end].iter().all(|spring| spring.damaged())
        && springs[end..]
            .iter()
            .all(|&spring| spring != Spring::Damaged)
}

fn valid_run(springs: &[Spring]) -> bool {
    springs[0..springs.len() - 1]
        .iter()
        .all(|spring| spring.damaged())
        && springs[springs.len() - 1].fine()
}

pub fn part_one(data: &str) -> Result<i64> {
    data.lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(springs, totals)| {
            let mut runs = totals
                .split(',')
                .map(|total| total.parse::<usize>())
                .collect::<Result<VecDeque<_>, _>>()
                .with_context(|| "invalid total")?;

            let springs: Vec<Spring> = springs.chars().map(|spring| Spring::from(spring)).collect();

            Ok(possibilities(&mut runs, &springs[..]))
        })
        .sum::<Result<i64>>()
}

pub fn part_two(_input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_sample() -> Result<()> {
        let sample: &str = concat!(
            "???.### 1,1,3\n",
            ".??..??...?##. 1,1,3\n",
            "?#?#?#?#?#?#?#? 1,3,1,6\n",
            "????.#...#... 4,1,1\n",
            "????.######..#####. 1,6,5\n",
            "?###???????? 3,2,1"
        );

        assert_eq!(part_one(sample)?, 21);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/12.actual");
        assert_eq!(part_one(input)?, 7307);
        Ok(())
    }

    #[test]
    fn part_2_sample() -> Result<()> {
        let sample: &str = concat!(
            "???.### 1,1,3\n",
            ".??..??...?##. 1,1,3\n",
            "?#?#?#?#?#?#?#? 1,3,1,6\n",
            "????.#...#... 4,1,1\n",
            "????.######..#####. 1,6,5\n",
            "?###???????? 3,2,1"
        );

        assert_eq!(part_two(sample)?, 0);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/11.actual");
        assert_eq!(part_two(input)?, 0);
        Ok(())
    }
}
