use anyhow::Result;

pub fn part_one(data: &str) -> usize {
    if let Some(width) = data.find("\n") {
        let data: Vec<char> = data.split("\n").flat_map(|row| row.chars()).collect();
        let length = data.len();
        let height = length / width;

        let mut tilted = vec![];

        for (mut i, &c) in data.iter().enumerate() {
            tilted.push(c);

            if c == 'O' {
                while let Some(j) = i.checked_sub(width) {
                    if tilted[j] == '.' {
                        tilted[j] = 'O';
                        tilted[i] = '.';
                        i = j;
                    } else {
                        break;
                    }
                }
            }
        }

        tilted
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| {
                if c == 'O' {
                    return Some(height - i / width);
                } else {
                    None
                }
            })
            .sum()
    } else {
        0
    }
}

pub fn part_two(_input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = concat!(
            "O....#....\n",
            "O.OO#....#\n",
            ".....##...\n",
            "OO.#O....O\n",
            ".O.....O#.\n",
            "O.#..O.#.#\n",
            "..O..#O..O\n",
            ".......O..\n",
            "#....###..\n",
            "#OO..#....\n",
        );

        assert_eq!(part_one(input), 136);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/14");
        assert_eq!(part_one(input), 111_339);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = concat!(
            "O....#....\n",
            "O.OO#....#\n",
            ".....##...\n",
            "OO.#O....O\n",
            ".O.....O#.\n",
            "O.#..O.#.#\n",
            "..O..#O..O\n",
            ".......O..\n",
            "#....###..\n",
            "#OO..#....\n",
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
