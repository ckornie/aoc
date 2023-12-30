use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Bijection {
    start: i64,
    end: i64,
    offset: i64,
}

impl Bijection {
    fn location(&self, seed: i64) -> Option<i64> {
        if self.start <= seed && self.end > seed {
            Some(seed + self.offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Data {
    seeds: Vec<i64>,
    bijections: Vec<Vec<Bijection>>,
}

impl TryFrom<&str> for Data {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let lines: Vec<&str> = value.lines().collect();
        let seeds = (lines[0].get("seeds: ".len()..))
            .unwrap_or("")
            .split(' ')
            .map(|seed| seed.parse::<i64>().with_context(|| "cannot parse seeds"))
            .collect::<Result<Vec<i64>>>()?;

        let mut bijections: Vec<Vec<Bijection>> = vec![];
        let mut map: Vec<Bijection> = vec![];

        for i in 1..lines.len() {
            let line = lines[i];
            if !line.is_empty() {
                if line.ends_with(" map:") && !map.is_empty() {
                    bijections.push(map);
                    map = vec![];
                } else if let Some((Ok(codomain), Ok(domain), Ok(span))) = line
                    .split(' ')
                    .map(|number| number.parse::<i64>())
                    .collect_tuple()
                {
                    map.push(Bijection {
                        start: domain,
                        end: domain + span,
                        offset: codomain - domain,
                    });
                }
            }
        }

        map.sort_by(|a, b| a.start.cmp(&b.start));
        bijections.push(map);

        Ok(Data { seeds, bijections })
    }
}

struct PartOne {
    data: Data,
}

impl PartOne {
    fn lowest(&self) -> i64 {
        let mut lowest = i64::MAX;
        for &seed in &self.data.seeds {
            let mut seed = seed;
            for bijections in &self.data.bijections {
                for maps in bijections {
                    if let Some(location) = maps.location(seed) {
                        seed = location;
                        break;
                    }
                }
            }
            lowest = if lowest < seed { lowest } else { seed }
        }

        lowest
    }
}

struct PartTwo {
    data: Data,
}

impl PartTwo {
    fn seeds(&self) -> Vec<i64> {
        self.data
            .seeds
            .iter()
            .tuples::<(&i64, &i64)>()
            .map(|(&start, &length)| {
                let mut seeds: Vec<i64> = vec![];
                for i in start..start + length {
                    seeds.push(i);
                }
                seeds
            })
            .flatten()
            .collect()
    }

    fn lowest(&self) -> i64 {
        let mut lowest = i64::MAX;
        for seed in self.seeds() {
            let mut seed = seed;
            for bijections in &self.data.bijections {
                for maps in bijections {
                    if let Some(location) = maps.location(seed) {
                        seed = location;
                        break;
                    }
                }
            }
            lowest = if lowest < seed { lowest } else { seed }
        }

        lowest
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/05.example");
        let almanac = PartOne {
            data: Data::try_from(input)?,
        };
        assert_eq!(almanac.lowest(), 35);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/05.actual");
        let almanac = PartOne {
            data: Data::try_from(input)?,
        };
        assert_eq!(almanac.lowest(), 486_613_012);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/05.example");
        let almanac = PartTwo {
            data: Data::try_from(input)?,
        };
        assert_eq!(almanac.lowest(), 46);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/05.actual");
        let almanac = PartTwo {
            data: Data::try_from(input)?,
        };
        assert_eq!(almanac.lowest(), 46);
        Ok(())
    }
}
