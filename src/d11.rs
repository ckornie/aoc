use std::collections::HashSet;

const STAR: char = '#';

fn distances(input: &str, multiplier: usize) -> usize {
    let width = input.chars().position(|c| c == '\n').unwrap_or(0);
    let mut rows: HashSet<usize> = (0..width).collect();
    let mut columns: HashSet<usize> = (0..input.len() / width).collect();

    let galaxy: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(c, feature)| {
                    if feature == STAR {
                        rows.remove(&r);
                        columns.remove(&c);
                        Some((r, c))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let galaxy: Vec<(usize, usize)> = galaxy
        .iter()
        .map(|(r, c)| {
            (
                r + (multiplier * empty_count(&rows, r)),
                c + (multiplier * empty_count(&columns, c)),
            )
        })
        .collect();

    galaxy
        .iter()
        .enumerate()
        .map(|(i, &(r, c))| {
            galaxy
                .iter()
                .skip(i)
                .fold(0, |z, &(x, y)| z + r.abs_diff(x) + c.abs_diff(y))
        })
        .sum()
}

fn empty_count(empties: &HashSet<usize>, index: &usize) -> usize {
    empties.iter().filter(|&i| i < index).count()
}

pub fn part_one(input: &str) -> usize {
    distances(input, 1)
}

pub fn part_two(input: &str) -> usize {
    distances(input, 999_999)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/11.example");
        assert_eq!(part_one(input), 374);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/11.actual");
        assert_eq!(part_one(input), 9_647_174);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/11.example");
        assert_eq!(part_two(input), 82_000_210);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/11.actual");
        assert_eq!(part_two(input), 377_318_892_554);
        Ok(())
    }
}
