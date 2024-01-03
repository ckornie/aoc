use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Bijection {
    start: i64,
    end: i64,
    offset: i64,
}

#[derive(Debug)]
pub struct Location {
    result: i64,
    supremum: i64,
}

impl Bijection {
    fn location(&self, seed: i64) -> Option<Location> {
        if self.start <= seed && self.end > seed {
            Some(Location {
                result: seed + self.offset,
                supremum: self.end - seed,
            })
        } else if self.start > seed {
            Some(Location {
                result: seed,
                supremum: self.start - seed,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Data {
    seeds: Vec<i64>,
    mapping: Vec<Vec<Bijection>>,
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

        let mut mapping: Vec<Vec<Bijection>> = vec![];
        let mut map: Vec<Bijection> = vec![];

        for i in 1..lines.len() {
            let line = lines[i];
            if !line.is_empty() {
                if line.ends_with(" map:") && !map.is_empty() {
                    map.sort_by(|a, b| a.start.cmp(&b.start));
                    mapping.push(map);
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
        mapping.push(map);

        Ok(Data { seeds, mapping })
    }
}

pub fn part_one(data: Data) -> Option<i64> {
    data.seeds
        .iter()
        .map(|&seed| {
            data.mapping.iter().fold(seed, |seed, map| {
                map.iter()
                    .filter_map(|bijection| bijection.location(seed))
                    .min_by(|a, b| a.supremum.cmp(&b.supremum))
                    .map_or(seed, |location| location.result)
            })
        })
        .min()
}

pub fn part_two(data: Data) -> Option<i64> {
    data.seeds
        .iter()
        .tuples::<(&i64, &i64)>()
        .flat_map(|(&start, &length)| {
            let mut results = vec![];
            let mut seed = start;
            while seed < (start + length) {
                let (result, supremum) =
                    data.mapping
                        .iter()
                        .fold((seed, i64::MAX), |(seed, supremum), map| {
                            map.iter()
                                .filter_map(|bijection| bijection.location(seed))
                                .min_by(|a, b| a.supremum.cmp(&b.supremum))
                                .map_or((seed, supremum), |location| {
                                    (location.result, supremum.min(location.supremum))
                                })
                        });
                results.push(result);
                seed = seed + supremum;
            }
            results
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/05.example");
        assert_eq!(part_one(Data::try_from(input)?), Some(35));
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/05.actual");
        assert_eq!(part_one(Data::try_from(input)?), Some(486_613_012));
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/05.example");
        assert_eq!(part_two(Data::try_from(input)?), Some(46));
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/05.actual");
        assert_eq!(part_two(Data::try_from(input)?), Some(56931769));
        Ok(())
    }
}
