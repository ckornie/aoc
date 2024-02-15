use std::{cmp, collections::HashMap};

fn compare(left: &[char], right: &[char]) -> usize {
    let mut count = 0;
    for i in 0..cmp::min(left.len(), right.len()) {
        if left[left.len() - i - 1] != right[i] {
            count = count + 1;
        }
    }
    count
}

fn mirror(data: &Vec<Vec<char>>, width: usize, errors: usize) -> Option<usize> {
    let mut counts: HashMap<usize, usize> = (1..width).map(|i| (i, 0)).collect();
    for row in data.iter() {
        for i in 1..width {
            let compare = compare(&row[..i], &row[i..]);
            counts.entry(i).and_modify(|count| (*count += compare));
        }
    }

    if let &[index] = counts
        .into_iter()
        .filter(|&(_, v)| v == errors)
        .map(|(k, _)| k)
        .collect::<Vec<usize>>()
        .as_slice()
    {
        Some(index)
    } else {
        None
    }
}

pub fn symmetry(data: &str, errors: usize) -> usize {
    let mut result = 0;

    if let Some(width) = data.find("\n") {
        let data: Vec<char> = data.split("\n").flat_map(|row| row.chars()).collect();
        let length = data.len();

        let first: Vec<Vec<char>> = (0..length / width)
            .map(|i| (0..width).map(|j| data[(i * width) + j]).collect())
            .collect();

        result = mirror(&first, width, errors)
            .map(|value| result + value)
            .unwrap_or(result);

        let second: Vec<Vec<char>> = (0..width)
            .map(|i| (0..length / width).map(|j| data[i + (j * width)]).collect())
            .collect();

        result = mirror(&second, length / width, errors)
            .map(|value| result + value * 100)
            .unwrap_or(result);
    }

    result
}

pub fn part_one(data: &str) -> usize {
    data.split("\n\n").map(|chart| symmetry(chart, 0)).sum()
}

pub fn part_two(data: &str) -> usize {
    data.split("\n\n").map(|chart| symmetry(chart, 1)).sum()
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

        assert_eq!(part_two(input), 400);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/13");
        assert_eq!(part_two(input), 37_617);
        Ok(())
    }
}
