use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;

const SPACER: char = '.';
const GEAR: char = '*';

#[derive(Debug)]
pub struct Part {
    number: u32,
}

impl TryFrom<Vec<&char>> for Part {
    type Error = anyhow::Error;

    fn try_from(value: Vec<&char>) -> std::result::Result<Self, self::Error> {
        String::from_iter(value)
            .parse::<u32>()
            .map(|number| Part { number })
            .with_context(|| format!("could not parse part number"))
    }
}

#[derive(Debug)]
pub struct Marker {
    symbol: char,
    parts: Vec<Part>,
    row: usize,
    column: usize,
}

struct Schematic {
    lines: Vec<Vec<char>>,
}

impl Schematic {
    fn scan(&self, row: usize, column: usize) -> Result<Vec<Part>> {
        [self.line(row, -1), self.line(row, 0), self.line(row, 1)]
            .into_iter()
            .filter_map(|line| line)
            .map(|line| self.parts(line, column))
            .flatten_ok()
            .collect::<Result<Vec<Part>>>()
    }

    fn line(&self, row: usize, offset: isize) -> Option<&Vec<char>> {
        row.checked_add_signed(offset)
            .map_or(None, |i| self.lines.get(i))
    }

    fn parts(&self, row: &Vec<char>, offset: usize) -> Result<Vec<Part>> {
        let mut left: Vec<&char> = row
            .iter()
            .rev()
            .skip(row.len() - offset)
            .take_while(|e| e.is_digit(10))
            .collect();
        left.reverse();

        let right: Vec<&char> = row
            .iter()
            .skip(offset + 1)
            .take_while(|e| e.is_digit(10))
            .collect();

        row.get(offset).map_or_else(
            || bail!("expected character"),
            |character| {
                if character.is_digit(10) {
                    left.push(character);
                    left.extend(right);
                    Part::try_from(left).map(|part| vec![part])
                } else {
                    [Part::try_from(left), Part::try_from(right)]
                        .into_iter()
                        .filter(|e| e.is_ok())
                        .collect()
                }
            },
        )
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        Schematic {
            lines: value
                .split('\n')
                .map(|line| line.chars().collect())
                .collect(),
        }
    }
}

impl IntoIterator for Schematic {
    type Item = Result<Marker>;
    type IntoIter = SchematicIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        SchematicIntoIterator {
            schematic: self,
            row: 0,
            column: 0,
        }
    }
}

pub struct SchematicIntoIterator {
    schematic: Schematic,
    row: usize,
    column: usize,
}

impl Iterator for SchematicIntoIterator {
    type Item = Result<Marker>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.row;
        let column = self.column;

        match self.schematic.lines.get(row) {
            Some(line) => match line.get(column) {
                Some(&symbol) => {
                    self.column = self.column + 1;
                    if !symbol.is_digit(10) && symbol != SPACER {
                        Some(self.schematic.scan(row, column).map(|parts| Marker {
                            symbol,
                            parts,
                            row,
                            column,
                        }))
                    } else {
                        self.next()
                    }
                }
                None => {
                    self.row = self.row + 1;
                    self.column = 0;
                    self.next()
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let input = include_str!("../res/03.example");
        let actual: Vec<u32> = Schematic::from(input)
            .into_iter()
            .map(|result| result.map(|marker| marker.parts))
            .flatten_ok()
            .map(|result| result.map(|part| part.number))
            .collect::<Result<Vec<u32>>>()?;
        assert_eq!(actual.into_iter().sum::<u32>(), 4361);
        Ok(())
    }

    #[test]
    fn solution_1() -> Result<()> {
        let input = include_str!("../res/03.actual");
        let actual: Vec<u32> = Schematic::from(input)
            .into_iter()
            .map(|result| result.map(|marker| marker.parts))
            .flatten_ok()
            .map(|result| result.map(|part| part.number))
            .collect::<Result<Vec<u32>>>()?;
        assert_eq!(actual.into_iter().sum::<u32>(), 538_046);
        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let input = include_str!("../res/03.example");
        let actual: Vec<u32> = Schematic::from(input)
            .into_iter()
            .filter_ok(|marker| marker.symbol.eq(&GEAR) && marker.parts.len() == 2)
            .map(|result| {
                result.map(|marker| marker.parts.iter().map(|parts| parts.number).product())
            })
            .collect::<Result<Vec<u32>>>()?;
        assert_eq!(actual.into_iter().sum::<u32>(), 467_835);
        Ok(())
    }

    #[test]
    fn solution_2() -> Result<()> {
        let input = include_str!("../res/03.actual");
        let actual: Vec<u32> = Schematic::from(input)
            .into_iter()
            .filter_ok(|marker| marker.symbol.eq(&GEAR) && marker.parts.len() == 2)
            .map(|result| {
                result.map(|marker| marker.parts.iter().map(|parts| parts.number).product())
            })
            .collect::<Result<Vec<u32>>>()?;
        assert_eq!(actual.into_iter().sum::<u32>(), 81_709_807);
        Ok(())
    }
}
