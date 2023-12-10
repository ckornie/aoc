use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    id: u32,
    samples: Vec<HashMap<String, u32>>,
}

impl Game {
    fn try_new(line: &str) -> Result<Self> {
        let (id, samples) = line
            .split_once(':')
            .with_context(|| format!("parsing error on line: {:?}", line))?;
        Ok(Self {
            id: parse_id(id)?,
            samples: parse_samples(samples)?,
        })
    }
}

pub fn parse_games(input: &str) -> Result<Vec<Game>> {
    input
        .split_inclusive('\n')
        .map(|l| Game::try_new(l))
        .collect()
}

fn parse_samples(text: &str) -> Result<Vec<HashMap<String, u32>>> {
    text.split(';').map(|s| parse_sample(s)).collect()
}

fn parse_sample(text: &str) -> Result<HashMap<String, u32>> {
    text.split(',')
        .filter_map(|x| x.trim().split_once(' '))
        .map(|x| {
            let (a, b) = x;
            if let Ok(c) = a.parse::<u32>() {
                Ok((b.to_string(), c))
            } else {
                Err(anyhow!(format!("parsing error on line: {:?}", text)))
            }
        })
        .collect()
}

fn parse_id(text: &str) -> Result<u32> {
    text["Game ".len()..]
        .parse::<u32>()
        .with_context(|| format!("couldn't find game id in {}", text))
}

pub fn sum_power(games: Vec<Game>) -> u32 {
    games
        .iter()
        .map(|g| {
            let mut power: HashMap<&String, u32> = HashMap::new();
            g.samples.iter().for_each(|s| {
                s.iter().for_each(|(k, v)| {
                    power
                        .entry(k)
                        .and_modify(|e| {
                            if *e < *v {
                                *e = *v
                            }
                        })
                        .or_insert(*v);
                });
            });

            power.iter().fold(1, |c, (_, v)| v * c)
        })
        .sum()
}

pub fn check_validity(games: Vec<Game>, constraints: &HashMap<&str, u32>) -> u32 {
    games
        .iter()
        .filter(|game| {
            !constraints.iter().any(|(k, v)| {
                game.samples
                    .iter()
                    .any(|sample| sample.get(k as &str).is_some_and(|y| y > v))
            })
        })
        .map(|game| game.id)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<()> {
        let constraints: HashMap<&str, u32> =
            HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let input = include_str!("../res/02.txt");
        assert_eq!(check_validity(parse_games(input)?, &constraints), 1734);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let input = include_str!("../res/02.txt");
        assert_eq!(sum_power(parse_games(input)?), 70387);
        Ok(())
    }
}
