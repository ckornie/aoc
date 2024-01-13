use anyhow::{bail, Context, Error, Result};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Waypoint {
    key: [char; 3],
    left: [char; 3],
    right: [char; 3],
}

impl TryFrom<&str> for Waypoint {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (key, directions) = value
            .split_once(" = (")
            .with_context(|| "unexpected file format")?;
        let (left, right) = directions
            .trim_end_matches(')')
            .split_once(", ")
            .with_context(|| "unexpected file format")?;

        let key: [char; 3] = key.chars().collect::<Vec<char>>().try_into().unwrap();
        let left: [char; 3] = left.chars().collect::<Vec<char>>().try_into().unwrap();
        let right: [char; 3] = right.chars().collect::<Vec<char>>().try_into().unwrap();

        Ok(Waypoint { key, left, right })
    }
}

#[derive(Debug)]
pub struct Map {
    instructions: Vec<char>,
    waypoints: HashMap<[char; 3], Waypoint>,
}

impl TryFrom<&str> for Map {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut lines = value.lines();

        let instructions = lines
            .next()
            .with_context(|| "unexpected file format")?
            .chars()
            .collect();

        let waypoints = lines
            .filter(|l| !l.is_empty())
            .map(|l| Waypoint::try_from(l))
            .map(|w| w.map(|w| (w.key, w)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Map {
            instructions,
            waypoints,
        })
    }
}

pub fn part_one(input: &str) -> Result<i64> {
    let map = Map::try_from(input)?;
    let terminus: [char; 3] = ['Z', 'Z', 'Z'];
    let mut location = ['A', 'A', 'A'];
    for i in 0.. {
        let waypoint = map
            .waypoints
            .get(&location)
            .with_context(|| "location not found")?;

        location = if map.instructions[i % map.instructions.len()] == 'L' {
            waypoint.left
        } else {
            waypoint.right
        };

        if location == terminus {
            return i64::try_from(i)
                .with_context(|| "hops exceeded")
                .map(|i| i + 1);
        }
    }

    bail!("hops exceeded");
}

pub fn part_two(input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/08.example");
        assert_eq!(part_one(input)?, 2);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/08.actual");
        assert_eq!(part_one(input)?, 16_043);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/08.example");
        assert_eq!(part_two(input)?, 0);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/08.actual");
        assert_eq!(part_two(input)?, 0);
        Ok(())
    }
}
