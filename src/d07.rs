use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Draw {
    HighCard { cards: [i8; 5] },
    OnePair { cards: [i8; 5] },
    TwoPair { cards: [i8; 5] },
    ThreeKind { cards: [i8; 5] },
    FullHouse { cards: [i8; 5] },
    FourKind { cards: [i8; 5] },
    FiveKind { cards: [i8; 5] },
}

impl TryFrom<&str> for Draw {
    type Error = anyhow::Error;

    fn try_from(hand: &str) -> std::result::Result<Self, self::Error> {
        let mapping = HashMap::from([
            ('2', 0),
            ('3', 1),
            ('4', 2),
            ('5', 3),
            ('6', 4),
            ('7', 5),
            ('8', 6),
            ('9', 7),
            ('T', 8),
            ('J', 9),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
        ]);

        let mut counts: HashMap<char, i8> = mapping.iter().map(|(&k, v)| (k, 0i8)).collect();

        let mut draw = [-1; 5];
        for (i, c) in hand.chars().enumerate() {
            match counts.get(&c) {
                Some(v) => counts.insert(c, v + 1),
                None => bail!("could not find count for {}", c),
            };
            match mapping.get(&c) {
                Some(v) => draw[i] = *v,
                None => bail!("could not find mapping for {}", c),
            };
        }

        let mut counts: Vec<i8> = counts.iter().map(|(_key, &value)| value).collect();
        counts.sort_by(|a, b| b.cmp(a));

        let counts: (i8, i8) = match counts.into_iter().next_tuple() {
            Some(v) => v,
            None => bail!("could not create top two counts"),
        };

        match counts {
            (5, 0) => Ok(Draw::FiveKind { cards: draw }),
            (4, 1) => Ok(Draw::FourKind { cards: draw }),
            (3, 2) => Ok(Draw::FullHouse { cards: draw }),
            (3, _) => Ok(Draw::ThreeKind { cards: draw }),
            (2, 2) => Ok(Draw::TwoPair { cards: draw }),
            (2, _) => Ok(Draw::OnePair { cards: draw }),
            _ => Ok(Draw::HighCard { cards: draw }),
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    draw: Draw,
    bid: i64,
}

#[derive(Debug)]
pub struct Data {
    hands: Vec<Hand>,
}

impl TryFrom<&str> for Data {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut hands: Vec<Hand> = value
            .lines()
            .map(|line| {
                if let Some((cards, bid)) = line.split_once(' ') {
                    if let Ok(bid) = bid.parse::<i64>() {
                        if let Ok(draw) = Draw::try_from(cards) {
                            Some(Hand { draw, bid })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        hands.sort_by(|a, b| a.draw.cmp(&b.draw));

        Ok(Data { hands })
    }
}

pub fn part_one(data: &mut Data) -> i64 {
    data.hands
        .iter()
        .enumerate()
        .map(|(i, Hand { draw: _, bid })| bid * (i as i64 + 1))
        .sum()
}

pub fn part_two(data: Data) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/07.example");
        assert_eq!(part_one(&mut Data::try_from(input)?), 6440);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/07.actual");
        assert_eq!(part_one(&mut Data::try_from(input)?), 246_163_188);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/07.example");
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/07.actual");
        Ok(())
    }
}
