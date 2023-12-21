use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Bijection {
    domain: u64,
    codomain: u64,
    span: u64,
}

impl Bijection {
    fn map(&self, seed: u64) -> Option<u64> {
        if self.domain <= seed && self.domain + self.span > seed {
            if let Ok(change) = self.offset() {
                seed.checked_add_signed(change)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn offset(&self) -> Result<i64> {
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

pub fn parse_almanac(
    data: &str,
    parser: &dyn Fn(Option<&str>) -> Result<Vec<u64>>,
) -> Result<Almanac> {
    let lines: Vec<&str> = data.lines().collect();
    let seeds = parser(lines[0].get("seeds: ".len()..))?;
    let mut bijections: Vec<Vec<Bijection>> = vec![];
    let mut map: Vec<Bijection> = vec![];

    for i in 1..lines.len() {
        let line = lines[i];
        if !line.is_empty() {
            if line.ends_with(" map:") && !map.is_empty() {
                map.sort_by(|a, b| a.domain.cmp(&b.domain));
                bijections.push(map);
                map = vec![];
            } else if let Some((Ok(codomain), Ok(domain), Ok(span))) = line
                .split(' ')
                .map(|number| number.parse::<u64>())
                .collect_tuple()
            {
                map.push(Bijection {
                    domain,
                    codomain,
                    span,
                });
            }
        }
    }

    map.sort_by(|a, b| a.domain.cmp(&b.domain));
    bijections.push(map);
    consolidate(&bijections);

    Ok(Almanac { seeds, bijections })
}

fn consolidate(bijections: &Vec<Vec<Bijection>>) -> u64 {
    for map in bijections.iter().rev() {
        println!("{:?}", map);
    }

    0
}

fn parse_seeds(seeds: Option<&str>) -> Result<Vec<u64>> {
    if let Some(seeds) = seeds {
        seeds
            .split(' ')
            .map(|seed| seed.parse::<u64>().with_context(|| "cannot parse seeds"))
            .collect::<Result<Vec<u64>>>()
    } else {
        bail!("could not parse seeds")
    }
}

fn parse_seed_ranges(seeds: Option<&str>) -> Result<Vec<u64>> {
    let result: Vec<u64> = parse_seeds(seeds)?
        .into_iter()
        .tuples::<(u64, u64)>()
        .map(|(start, length)| {
            let mut seeds: Vec<u64> = vec![];
            for i in start..start + length {
                seeds.push(i);
            }
            seeds
        })
        .flatten()
        .collect();
    Ok(result)
}

pub fn lowest_location(almanac: Result<Almanac>) -> Result<u64> {
    let mut lowest = u64::MAX;
    if let Ok(almanac) = almanac {
        for seed in almanac.seeds {
            let mut seed = seed;
            for bijections in &almanac.bijections {
                for maps in bijections {
                    if let Some(x) = maps.map(seed) {
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
        assert_eq!(lowest_location(parse_almanac(input, &parse_seeds))?, 35);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/05.actual");
        assert_eq!(
            lowest_location(parse_almanac(input, &parse_seeds))?,
            486_613_012
        );
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/05.example");
        assert_eq!(
            lowest_location(parse_almanac(input, &parse_seed_ranges))?,
            46
        );
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/05.actual");
        Ok(())
    }
}
