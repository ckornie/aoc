use anyhow::Result;
use std::{cmp, collections::HashSet};

fn compare(left: &[char], right: &[char]) -> bool {
    for i in 0..cmp::min(left.len(), right.len()) {
        if left[left.len() - i - 1] != right[i] {
            return false;
        }
    }

    true
}

fn mirror(data: &Vec<Vec<char>>, width: usize) -> Option<usize> {
    let mut column: HashSet<usize> = (1..width).collect();
    for row in data.iter() {
        for i in 1..width {
            if !compare(&row[..i], &row[i..]) {
                column.remove(&i);
            }
        }
    }

    if let &[index] = column.into_iter().collect::<Vec<usize>>().as_slice() {
        Some(index)
    } else {
        None
    }
}

pub fn symmetry(data: &str) -> usize {
    let mut result = 0;

    if let Some(width) = data.find("\n") {
        let data: Vec<char> = data.split("\n").flat_map(|row| row.chars()).collect();
        let length = data.len();

        let first: Vec<Vec<char>> = (0..length / width)
            .map(|i| (0..width).map(|j| data[(i * width) + j]).collect())
            .collect();

        result = mirror(&first, width)
            .map(|value| result + value)
            .unwrap_or(result);

        let second: Vec<Vec<char>> = (0..width)
            .map(|i| (0..length / width).map(|j| data[i + (j * width)]).collect())
            .collect();

        result = mirror(&second, length / width)
            .map(|value| result + value * 100)
            .unwrap_or(result);
    }

    result
}

pub fn part_one(data: &str) -> usize {
    data.split("\n\n").map(|chart| symmetry(chart)).sum()
}

pub fn part_two(_data: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = concat!(
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
            "\n",
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        );

        assert_eq!(part_one(input), 405);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/13");
        assert_eq!(part_one(input), 31_956);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = concat!(
            "#.##..##.\n",
            "..#.##.#.\n",
            "##......#\n",
            "##......#\n",
            "..#.##.#.\n",
            "..##..##.\n",
            "#.#.##.#.\n",
            "\n",
            "#...##..#\n",
            "#....#..#\n",
            "..##..###\n",
            "#####.##.\n",
            "#####.##.\n",
            "..##..###\n",
            "#....#..#\n",
        );

        assert_eq!(part_two(input)?, 0);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/13");
        assert_eq!(part_two(input)?, 0);
        Ok(())
    }
}
