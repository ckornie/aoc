use anyhow::{Context, Result};
use std::collections::HashMap;

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

struct Evaluation {
    runs: Vec<usize>,
    springs: Vec<Spring>,
}

impl Evaluation {
    fn permutations(
        &self,
        run_idx: usize,
        spring_idx: usize,
        cache: &mut HashMap<(usize, usize), i64>,
    ) -> i64 {
        let key = (run_idx, spring_idx);

        if let Some(&result) = cache.get(&key) {
            result
        } else {
            let result = if self.required(run_idx) > self.springs.len() - spring_idx {
                0
            } else if let Some(&run) = self.runs.get(run_idx) {
                if run_idx == self.runs.len() - 1 {
                    if self.anchored(spring_idx) {
                        if self.run_of(spring_idx, run) && self.no_more(spring_idx, run) {
                            1
                        } else {
                            0
                        }
                    } else {
                        if self.run_of(spring_idx, run) && self.no_more(spring_idx, run) {
                            1 + self.permutations(run_idx, spring_idx + 1, cache)
                        } else {
                            self.permutations(run_idx, spring_idx + 1, cache)
                        }
                    }
                } else {
                    if self.anchored(spring_idx) {
                        if self.run_of(spring_idx, run) && self.fine(spring_idx, run) {
                            self.permutations(run_idx + 1, spring_idx + run + 1, cache)
                        } else {
                            0
                        }
                    } else {
                        if self.run_of(spring_idx, run) && self.fine(spring_idx, run) {
                            self.permutations(run_idx + 1, spring_idx + run + 1, cache)
                                + self.permutations(run_idx, spring_idx + 1, cache)
                        } else {
                            self.permutations(run_idx, spring_idx + 1, cache)
                        }
                    }
                }
            } else {
                0
            };

            cache.insert(key, result);
            result
        }
    }

    fn required(&self, idx: usize) -> usize {
        self.runs.iter().skip(idx).sum::<usize>() + self.runs.len() - idx - 1
    }

    fn anchored(&self, idx: usize) -> bool {
        self.springs
            .get(idx)
            .is_some_and(|&spring| spring == Spring::Damaged)
    }

    fn run_of(&self, idx: usize, run: usize) -> bool {
        self.springs
            .iter()
            .skip(idx)
            .take(run)
            .all(|spring| spring.damaged())
    }

    fn no_more(&self, idx: usize, run: usize) -> bool {
        self.springs
            .iter()
            .skip(idx + run)
            .all(|&spring| spring != Spring::Damaged)
    }

    fn fine(&self, idx: usize, run: usize) -> bool {
        self.springs
            .get(idx + run)
            .is_some_and(|spring| spring.fine())
    }
}

pub fn part_one(data: &str) -> Result<i64> {
    data.lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(springs, totals)| {
            let runs = totals
                .split(',')
                .map(|total| total.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| "invalid total")?;

            let springs: Vec<Spring> = springs.chars().map(|spring| Spring::from(spring)).collect();

            Ok(Evaluation { runs, springs }.permutations(0, 0, &mut HashMap::new()))
        })
        .sum::<Result<i64>>()
}

pub fn part_two(data: &str) -> Result<i64> {
    data.lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(springs, totals)| {
            let runs: Vec<usize> = totals
                .split(',')
                .map(|total| total.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| "invalid total")?;

            let runs: Vec<usize> = (0..5).flat_map(|_| &runs).copied().collect::<Vec<_>>();

            let springs: Vec<Spring> = springs
                .chars()
                .map(|spring| Spring::from(spring))
                .chain([Spring::Unknown].into_iter())
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect();

            Ok(Evaluation { runs, springs }.permutations(0, 0, &mut HashMap::new()))
        })
        .sum::<Result<i64>>()
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
        let input = include_str!("../res/12");
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

        assert_eq!(part_two(sample)?, 525_152);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/12");
        assert_eq!(part_two(input)?, 3_415_570_893_842);
        Ok(())
    }
}
