use anyhow::Result;
use std::collections::HashMap;
use suffix::SuffixTable;

fn main() -> Result<()> {
    let input = include_str!("../../res/01.txt");
    let mut first: Option<(char, u32)> = None;
    let mut last: Option<(char, u32)> = None;
    let mut values: Vec<u32> = Vec::new();

    let tokens: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
        ("1", '1'),
        ("2", '2'),
        ("3", '3'),
        ("4", '4'),
        ("5", '5'),
        ("6", '6'),
        ("7", '7'),
        ("8", '8'),
        ("9", '9'),
    ]);

    for line in input.split('\n') {
        let st = SuffixTable::new(line);

        for (token, value) in tokens.iter() {
            let mut positions = st.positions(token).to_owned();
            positions.sort();

            if let Some(lo) = positions.first() {
                if let Some((_, prior)) = first {
                    if lo < &prior {
                        first = Option::Some((*value, *lo));
                    }
                } else {
                    first = Option::Some((*value, *lo));
                }
            }

            if let Some(hi) = positions.last() {
                if let Some((_, prior)) = last {
                    if hi > &prior {
                        last = Option::Some((*value, *hi));
                    }
                } else {
                    last = Option::Some((*value, *hi));
                }
            }
        }

        if let Some((a, _)) = first {
            if let Some((b, _)) = last {
                values.push(format!("{}{}", a, b).parse::<u32>().unwrap());
            }
        }

        first = None;
        last = None;
    }

    let sum: u32 = values.iter().sum();
    println!("The total sum is: {}", sum);

    Ok(())
}
