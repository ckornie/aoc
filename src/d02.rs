use anyhow::{bail, Context, Result};
use std::collections::HashMap;

pub fn check_games(text: &str, check: &HashMap<&str, u32>) -> Result<u32> {
    text.split('\n')
        .map(parse_line)
        .for_each(|a| println!("{:?}", a));

    Ok(0)
}

fn parse_line(line: &str) -> Result<(u32, Vec<Vec<(&str, u32)>>)> {
    if let Some((a, b)) = line.split_once(':') {
        return Ok((parse_id(a)?, parse_samples(b)));
    }
    bail!("unexpected format {}", line);
}

fn parse_id(substring: &str) -> Result<u32> {
    substring
        .replace("Game ", "")
        .parse::<u32>()
        .with_context(|| format!("couldn't find game in {}", substring))
}

fn parse_samples(substring: &str) -> Vec<Vec<(&str, u32)>> {
    substring
        .split("; ")
        .map(|samples| {
            samples
                .split(", ")
                .filter_map(|x| parse_sample(x.split_once(' ')))
                .collect()
        })
        .collect()
}

fn parse_sample<'a>(pair: Option<(&'a str, &'a str)>) -> Option<(&str, u32)> {
    if let Some((count, colour)) = pair {
        if let Ok(count) = count.parse::<u32>() {
            return Some((colour, count));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<()> {
        let check: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let input = include_str!("../res/02.txt");
        assert_eq!(check_games(input, &check)?, 0);
        Ok(())
    }
}
