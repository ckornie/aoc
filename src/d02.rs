use anyhow::{bail, Context, Result};
use regex::Regex;
use std::collections::HashMap;

pub fn check_games(input: &str, constraints: &HashMap<&str, u32>) -> Result<u32> {
    let c = compile(constraints)?;

    Ok(input
        .split_inclusive('\n')
        .filter_map(|x| check_game(x, &c).ok())
        .sum())
}

fn compile(constraints: &HashMap<&str, u32>) -> Result<Vec<(Regex, u32)>> {
    let mut compiled: Vec<(Regex, u32)> = Vec::new();
    for (k, v) in constraints {
        let r: Regex = Regex::new((format!(r"(\d*) {}[;,\n]", k)).as_str())?;
        compiled.push((r, *v));
    }
    Ok(compiled)
}

fn check_game(line: &str, constraints: &Vec<(Regex, u32)>) -> Result<u32> {
    if let Some((left, right)) = line.split_once(':') {
        if check_samples(right, constraints)? {
            return parse_id(left).with_context(|| "parsing error");
        }
    }
    bail!("parsing error")
}

fn parse_id(text: &str) -> Result<u32> {
    text.replace("Game ", "")
        .parse::<u32>()
        .with_context(|| format!("couldn't find game in {}", text))
}

fn check_samples(text: &str, constraints: &Vec<(Regex, u32)>) -> Result<bool> {
    for (r, v) in constraints {
        let t = r
            .captures_iter(text)
            .filter_map(|m| m.get(1)?.as_str().parse::<u32>().ok())
            .any(|x| x > *v);
        if t {
            return Ok(false);
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<()> {
        let constraints: HashMap<&str, u32> =
            HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let input = include_str!("../res/02.txt");
        assert_eq!(check_games(input, &constraints)?, 1734);
        Ok(())
    }
}
