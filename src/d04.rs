use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;

#[derive(Debug)]
pub struct Card {
    id: u32,
    wins: HashSet<u32>,
    ours: HashSet<u32>,
}

impl Card {
    fn matches(&self) -> Vec<&u32> {
        self.wins.intersection(&self.ours).collect()
    }

    fn points(&self) -> u32 {
        let winners: Vec<&u32> = self.matches();
        if winners.len() > 0 {
            1 << winners.len() - 1
        } else {
            0
        }
    }

    fn cards(&self, tally: &mut HashMap<u32, u32>) -> u32 {
        let total = self.matches().len() as u32;
        let cards = *tally
            .entry(self.id)
            .and_modify(|e| *e = *e + 1)
            .or_insert(1);

        for i in self.id + 1..=self.id + total {
            tally
                .entry(i)
                .and_modify(|e| *e = *e + cards)
                .or_insert(cards);
        }
        cards
    }
}

fn parse_numbers(data: &str) -> Result<HashSet<u32>> {
    data.split(' ')
        .into_iter()
        .map(|number| number.trim())
        .filter(|number| !number.is_empty())
        .map(|number| {
            number
                .parse::<u32>()
                .with_context(|| format!("couldn't parse number {:?}", number))
        })
        .collect()
}

fn parse_cards(data: &str) -> Result<Vec<Card>> {
    data.split('\n')
        .filter(|card| !card.is_empty())
        .map(|card| match card.split_once(':') {
            Some((id, numbers)) => match numbers.split_once('|') {
                Some((wins, ours)) => Ok(Card {
                    id: id["Card ".len()..].trim().parse::<u32>()?,
                    wins: parse_numbers(wins)?,
                    ours: parse_numbers(ours)?,
                }),
                None => bail!("couldn't parse numbers"),
            },
            None => bail!("couldn't parse card"),
        })
        .collect()
}

pub fn points(data: &str) -> Result<u32> {
    Ok(parse_cards(data)?
        .iter()
        .map(|card| card.points())
        .sum::<u32>())
}

pub fn cards(data: &str) -> Result<u32> {
    let mut tally: HashMap<u32, u32> = HashMap::new();
    Ok(parse_cards(data)?
        .iter()
        .map(|card| card.cards(&mut tally))
        .sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/04.example");
        assert_eq!(points(input)?, 13);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/04.actual");
        assert_eq!(points(input)?, 21213);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/04.example");
        assert_eq!(cards(input)?, 30);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/04.actual");
        assert_eq!(cards(input)?, 8549735);
        Ok(())
    }
}
