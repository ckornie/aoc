use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::SplitInclusive,
};

#[derive(Debug)]
pub struct Mapping {
    from: String,
    to: String,
}

#[derive(Debug)]
pub struct Bijection {
    mapping: Mapping,
    domain: u64,
    codomain: u64,
    span: u64,
}

impl Bijection {
    fn contains(&self, seed: u64) -> Option<u64> {
        if self.domain <= seed && self.domain + self.span > seed {
            if let Ok(change) = self.change() {
                seed.checked_add_signed(change)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn change(&self) -> Result<i64> {
        let domain = i64::try_from(self.domain).with_context(|| "cannot cast domain")?;
        let codomain = i64::try_from(self.codomain).with_context(|| "cannot cast codomain")?;
        Ok(codomain - domain)
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    bijections: Vec<Vec<Bijection>>,
}

impl TryFrom<Option<(&str, &str)>> for Mapping {
    type Error = anyhow::Error;

    fn try_from(value: Option<(&str, &str)>) -> Result<Self, Self::Error> {
        if let Some((from, to)) = value {
            Ok(Mapping {
                from: String::from(from),
                to: String::from(to),
            })
        } else {
            bail!("cannot create mapping")
        }
    }
}

fn parse_almanac(data: &str) -> Result<Almanac> {
    let lines: Vec<&str> = data.lines().collect();
    let seeds: Vec<u64> = lines[0].get("seeds: ".len()..).map_or(vec![], |x| {
        x.split(' ').map(|e| e.parse::<u64>()).flatten().collect()
    });

    let mut heading: &str = "";
    let mut mapping: Option<Mapping> = None;
    let mut bijections: Vec<Vec<Bijection>> = vec![];
    let mut active = vec![];

    for i in 1..lines.len() {
        let line = lines[i];
        if let Some(prefix) = line.strip_suffix(" map:") {
            heading = prefix;
            bijections.push(active);
            active = vec![];
        } else if let Some((a, b, c)) = line
            .split(' ')
            .map(|e| e.parse::<u64>())
            .flatten()
            .collect_tuple()
        {
            let bijection = Bijection {
                mapping: Mapping::try_from(heading.split_once("-to-")).ok().unwrap(),
                domain: b,
                codomain: a,
                span: c,
            };

            active.push(bijection);
        }
    }

    bijections.push(active);

    Ok(Almanac { seeds, bijections })
}

pub fn lowest_location(almanac: Result<Almanac>) -> Result<u64> {
    let mut lowest = u64::MAX;
    if let Ok(almanac) = almanac {
        for e in almanac.seeds {
            let mut seed = e;
            for f in &almanac.bijections {
                for g in f {
                    if let Some(x) = g.contains(seed) {
                        seed = x;
                        break;
                    }
                }
            }

            lowest = if lowest < seed { lowest } else { seed }
        }
    }
    Ok(lowest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/05.example");
        assert_eq!(lowest_location(parse_almanac(input))?, 35);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/05.actual");
        assert_eq!(lowest_location(parse_almanac(input))?, 486_613_012);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        Ok(())
    }
}
