use anyhow::{anyhow, Context, Error, Result};

const SPACER: char = '.';

#[derive(Debug)]
pub struct Part {
    number: Vec<char>,
    row: usize,
    column: usize,
}

impl Part {
    fn left_bound(&self) -> usize {
        self.column.checked_add_signed(-1).unwrap_or(0)
    }

    fn right_bound(&self) -> usize {
        if self.column == 0 {
            self.number.len()
        } else {
            self.column + self.number.len()
        }
    }
}

struct Schematic {
    lines: Vec<Vec<char>>,
}

impl Schematic {
    fn validate(&self, part: &Part) -> bool {
        self.neighbours(part)
            .iter()
            .any(|c| !c.is_digit(10) && *c != SPACER)
    }

    fn neighbours(&self, part: &Part) -> Vec<char> {
        self.adjacent(part, 0)
            .into_iter()
            .chain(self.adjacent(part, -1).into_iter())
            .chain(self.adjacent(part, 1).into_iter())
            .collect()
    }

    fn adjacent(&self, part: &Part, offset: isize) -> Vec<char> {
        part.row.checked_add_signed(offset).map_or(vec![], |v| {
            self.lines.get(v).map_or(vec![], |line| {
                line.iter()
                    .skip(part.left_bound())
                    .take(part.right_bound() - part.left_bound() + 1)
                    .cloned()
                    .collect()
            })
        })
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
    type Item = Part;
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

impl SchematicIntoIterator {
    fn read_number(&mut self, buffer: &mut Vec<char>) {
        match self.schematic.lines.get(self.row) {
            Some(line) => match line.get(self.column) {
                Some(character) => {
                    self.column = self.column + 1;
                    if character.is_digit(10) {
                        buffer.push(*character);
                        self.read_number(buffer)
                    }
                }
                None => {
                    self.row = self.row + 1;
                    self.column = 0;
                }
            },
            None => (),
        }
    }
}

impl Iterator for SchematicIntoIterator {
    type Item = Part;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.row;
        let column = self.column;

        match self.schematic.lines.get(row) {
            Some(line) => match line.get(column) {
                Some(character) => {
                    self.column = self.column + 1;
                    if character.is_digit(10) {
                        let mut number = vec![*character];
                        self.read_number(&mut number);

                        let part = Part {
                            number,
                            row,
                            column,
                        };

                        if self.schematic.validate(&part) {
                            Some(part)
                        } else {
                            self.next()
                        }
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
    use std::fs;

    #[test]
    fn example_part_1() -> Result<()> {
        let input = include_str!("../res/03.example");
        let actual: Vec<u32> = Schematic::from(input)
            .into_iter()
            .map(|p| String::from_iter(p.number))
            .map(|s| s.parse::<u32>().with_context(|| "failed to parse"))
            .collect::<Result<Vec<u32>>>()?;
        assert_eq!(actual.into_iter().sum::<u32>(), 4361);
        Ok(())
    }

    #[test]
    fn part_1() -> Result<()> {
        let input = include_str!("../res/03.actual");
        let schematic = Schematic::from(input);
        let actual: Vec<u32> = schematic
            .into_iter()
            .map(|p| String::from_iter(p.number))
            .map(|s| s.parse::<u32>().with_context(|| "failed to parse"))
            .collect::<Result<Vec<u32>>>()?;
        assert_eq!(actual.into_iter().sum::<u32>(), 538046);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let input = include_str!("../res/03.actual");
        let schematic = Schematic::from(input);
        let actual: Vec<String> = schematic
            .into_iter()
            .map(|p| String::from_iter(p.number))
            .collect();
        fs::write("/tmp/parts.txt", actual.join("\n"))?;
        Ok(())
    }
}
