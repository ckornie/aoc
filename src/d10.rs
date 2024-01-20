use anyhow::{bail, Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    T = 0b0001,
    R = 0b0010,
    B = 0b0100,
    L = 0b1000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tile {
    //     LBRTLBRT
    BL = 0b00000000,
    ST = 0b11111111,
    NS = 0b01010101,
    EW = 0b10101010,
    NE = 0b11000011,
    NW = 0b01101001,
    SE = 0b10010110,
    SW = 0b00111100,
}

impl Tile {
    fn connected(self, neighbour: Tile, direction: Direction) -> bool {
        ((direction as u32 & self as u32) << 4) & neighbour as u32 > 0
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            'S' => Tile::ST,
            _ => Tile::BL,
        }
    }
}

struct Network {
    tiles: Vec<Vec<Tile>>,
    row: usize,
    column: usize,
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = vec![];
        let mut row = 0;
        let mut column = 0;
        for (i, line) in value.lines().enumerate() {
            let mut pending = vec![];
            for (j, character) in line.chars().enumerate() {
                let tile = Tile::from(character);
                if tile == Tile::ST {
                    row = i;
                    column = j;
                }
                pending.push(tile);
            }
            tiles.push(pending);
        }

        Network { tiles, row, column }
    }
}

impl Network {
    fn connected(&mut self, direction: Direction) -> Option<Tile> {
        match direction {
            Direction::T => self.offset(-1, 0),
            Direction::R => self.offset(0, 1),
            Direction::B => self.offset(1, 0),
            Direction::L => self.offset(0, -1),
        }
        .and_then(|(neighbour, row, column)| {
            let tile = self.tiles[self.row][self.column];
            if tile.connected(neighbour, direction) {
                self.row = row;
                self.column = column;
                Some(neighbour)
            } else {
                None
            }
        })
    }

    fn offset(&self, row: isize, column: isize) -> Option<(Tile, usize, usize)> {
        let row = self.row.checked_add_signed(row);
        let column = self.column.checked_add_signed(column);
        if let Some((row, column)) = row.and_then(|r| column.map(|c| (r, c))) {
            self.tiles
                .get(row)
                .and_then(|r| r.get(column))
                .map(|&tile| (tile, row, column))
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Result<usize> {
    let mut network = Network::from(input);
    let directions: [Direction; 4] = [Direction::T, Direction::R, Direction::B, Direction::L];
    let mut i = 0;
    let mut count = 0;
    loop {
        let direction = directions[i];
        if let Some(tile) = network.connected(direction) {
            count = count + 1;

            if tile == Tile::ST {
                return Ok(count / 2);
            }

            i = (i + directions.len() - 1) % directions.len();
        } else {
            i = (i + 1) % directions.len();
        }
    }
}

pub fn part_two(input: &str) -> Result<i64> {
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/10.example");
        assert_eq!(part_one(input)?, 8);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/10.actual");
        assert_eq!(part_one(input)?, 6903);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/10.example");
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/10.actual");
        Ok(())
    }
}
