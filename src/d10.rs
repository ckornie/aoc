use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    T = 0b0001,
    R = 0b0010,
    B = 0b0100,
    L = 0b1000,
}

impl Direction {
    fn next(self) -> Direction {
        match self {
            Direction::T => Direction::R,
            Direction::R => Direction::B,
            Direction::B => Direction::L,
            Direction::L => Direction::T,
        }
    }

    fn previous(self) -> Direction {
        self.next().next().next()
    }

    fn pipe(self, direction: Direction) -> Option<Pipe> {
        match (self, direction) {
            (Direction::T, Direction::B) => Some(Pipe::NS),
            (Direction::T, Direction::R) => Some(Pipe::NE),
            (Direction::T, Direction::L) => Some(Pipe::NW),
            (Direction::R, Direction::B) => Some(Pipe::SE),
            (Direction::R, Direction::L) => Some(Pipe::EW),
            (Direction::L, Direction::B) => Some(Pipe::SW),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Pipe {
    BL = 0b00000000,
    ST = 0b11111111,
    NS = 0b01010101,
    EW = 0b10101010,
    NE = 0b11000011,
    NW = 0b01101001,
    SE = 0b10010110,
    SW = 0b00111100,
}

impl Pipe {
    fn connected(self, neighbour: Pipe, direction: Direction) -> bool {
        ((direction as u32 & self as u32) << 4) & neighbour as u32 > 0
    }

    fn northbound(self) -> bool {
        Direction::T as u32 & self as u32 > 0
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            'S' => Pipe::ST,
            _ => Pipe::BL,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    row: usize,
    column: usize,
    pipe: Pipe,
    origin: bool,
    connected: bool,
}

impl Tile {
    fn new(row: usize, column: usize, pipe: char) -> Tile {
        let pipe = Pipe::from(pipe);
        Tile {
            row,
            column,
            pipe,
            origin: pipe == Pipe::ST,
            connected: pipe == Pipe::ST,
        }
    }

    fn connect(self) -> Tile {
        Tile {
            connected: true,
            ..self
        }
    }

    fn link(self, first: Direction, second: Direction) -> Tile {
        let pipe = first
            .pipe(second)
            .or(second.pipe(first))
            .unwrap_or(Pipe::BL);

        Tile { pipe, ..self }
    }

    fn index(&self) -> (usize, usize) {
        (self.row, self.column)
    }
}

fn map(data: &str) -> Vec<Vec<Tile>> {
    data.lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, pipe)| Tile::new(i, j, pipe))
                .collect::<Vec<Tile>>()
        })
        .collect()
}

fn origin(map: &Vec<Vec<Tile>>) -> Option<&Tile> {
    map.iter()
        .flat_map(|row| row.iter())
        .filter(|tile| tile.origin)
        .next()
}

fn neighbour(tiles: &Vec<Vec<Tile>>, tile: Tile, direction: Direction) -> (Direction, Tile) {
    match direction {
        Direction::T => clip(tiles, tile.index(), -1, 0),
        Direction::R => clip(tiles, tile.index(), 0, 1),
        Direction::B => clip(tiles, tile.index(), 1, 0),
        Direction::L => clip(tiles, tile.index(), 0, -1),
    }
    .map_or_else(
        || neighbour(tiles, tile, direction.next()),
        |&candidate| {
            if tile.pipe.connected(candidate.pipe, direction) {
                (direction, candidate.connect())
            } else {
                neighbour(tiles, tile, direction.next())
            }
        },
    )
}

fn clip(tiles: &Vec<Vec<Tile>>, index: (usize, usize), row: isize, column: isize) -> Option<&Tile> {
    let (r, c) = index;
    let row = r.checked_add_signed(row);
    let column = c.checked_add_signed(column);
    if let Some((row, column)) = row.and_then(|r| column.map(|c| (r, c))) {
        tiles.get(row).and_then(|r| r.get(column))
    } else {
        None
    }
}

fn create(input: &str) -> Result<Vec<Vec<Tile>>> {
    let mut map = map(input);
    if let Some(&origin) = origin(&map) {
        let mut direction = Direction::T;
        let mut tile = origin;

        let (first, _) = neighbour(&map, tile, direction);
        let (last, _) = neighbour(&map, tile, first.next());
        map[tile.row][tile.column] = tile.link(first, last);

        loop {
            (direction, tile) = neighbour(&map, tile, direction.previous());
            if tile.origin {
                return Ok(map);
            } else {
                map[tile.row][tile.column] = tile;
            }
        }
    }

    bail!("could not locate origin")
}

pub fn part_one(input: &str) -> Result<usize> {
    let map = create(input)?;
    let network: Vec<&Tile> = map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| tile.connected)
        .collect();
    Ok((network.len() + 1) / 2)
}

pub fn part_two(input: &str) -> Result<usize> {
    let map = create(input)?;
    let mut count = 0;
    for row in map {
        let mut breached = false;
        let mut pending = 0;
        for tile in row {
            if tile.connected {
                if tile.pipe.northbound() {
                    breached = !breached;
                    count = count + pending;
                    pending = 0;
                }
            } else if breached {
                pending = pending + 1;
            }
        }
    }

    Ok(count)
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
        assert_eq!(part_two(input)?, 1);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/10.actual");
        assert_eq!(part_two(input)?, 265);
        Ok(())
    }
}
