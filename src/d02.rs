use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game<'a> {
    id: u32,
    samples: Vec<HashMap<&'a str, u32>>,
}

impl<'a> Game<'a> {
    fn try_new(line: &'a str) -> Result<Self> {
        let (id, samples) = line
            .split_once(':')
            .with_context(|| format!("parsing error on line: {:?}", line))?;
        Ok(Self {
            id: parse_id(id)?,
            samples: parse_samples(samples)?,
        })
    }
}

pub fn parse_games<'a>(input: &'a str) -> Result<Vec<Game<'a>>> {
    input
        .split_inclusive('\n')
        .map(|l| Game::try_new(l))
        .collect()
}

fn parse_samples<'a>(text: &'a str) -> Result<Vec<HashMap<&'a str, u32>>> {
    text.split(';').map(|s| parse_sample(s)).collect()
}

fn parse_sample<'a>(text: &'a str) -> Result<HashMap<&'a str, u32>> {
    text.split(',')
        .filter_map(|x| x.trim().split_once(' '))
        .map(|x| {
            let (a, b) = x;
            if let Ok(c) = a.parse::<u32>() {
                Ok((b, c))
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

pub fn check_validity<'a>(games: Vec<Game<'a>>, constraints: &HashMap<&str, u32>) -> u32 {
    games
        .iter()
        .filter(|game| {
            !constraints.iter().any(|(k, v)| {
                game.samples
                    .iter()
                    .any(|sample| sample.get(k).is_some_and(|y| y > v))
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
}
