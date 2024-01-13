use anyhow::{anyhow, bail, Context, Result};
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

        let key: [char; 3] = key
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .map_err(|_| anyhow!("invalid key"))?;
        let left: [char; 3] = left
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .map_err(|_| anyhow!("invalid left value"))?;
        let right: [char; 3] = right
            .chars()
            .collect::<Vec<char>>()
            .try_into()
            .map_err(|_| anyhow!("invalid right value"))?;

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

pub fn part_one(input: &str) -> Result<usize> {
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
            return Ok(i + 1);
        }
    }

    bail!("hops exceeded");
}

pub fn part_two(input: &str) -> Result<usize> {
    let map = Map::try_from(input)?;
    let mut locations: Vec<[char; 3]> = map
        .waypoints
        .iter()
        .map(|(k, _)| *k)
        .filter(|e| e.ends_with(&['A']))
        .collect();

    let mut intervals: Vec<usize> = vec![];

    for location in &mut locations {
        for i in 0.. {
            let instruction = map.instructions[i % map.instructions.len()];
            let waypoint = map
                .waypoints
                .get(location)
                .with_context(|| "location not found")?;

            if instruction == 'L' {
                location.clone_from(&waypoint.left);
            } else {
                location.clone_from(&waypoint.right);
            };

            if location.ends_with(&['Z']) {
                intervals.push(i + 1);
                break;
            }
        }
    }

    let product: usize = intervals
        .iter()
        .map(|&interval| {
            if interval % map.instructions.len() == 0 {
                interval / map.instructions.len()
            } else {
                interval
            }
        })
        .product();

    Ok(product * map.instructions.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/08-1.example");
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
        let input = include_str!("../res/08-2.example");
        assert_eq!(part_two(input)?, 6);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/08.actual");
        assert_eq!(part_two(input)?, 15_726_453_850_399);
        Ok(())
    }
}
