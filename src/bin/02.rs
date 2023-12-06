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

            let lo = positions.first();
            if lo.is_some() {
                if first.is_some() {
                    let (_, prior) = first.unwrap();
                    if lo.unwrap().lt(&prior) {
                        first = Option::Some((*value, *lo.unwrap()));
                    }
                } else {
                    first = Option::Some((*value, *lo.unwrap()));
                }
            }

            let hi = positions.last();
            if hi.is_some() {
                if last.is_some() {
                    let (_, prior) = last.unwrap();
                    if hi.unwrap().gt(&prior) {
                        last = Option::Some((*value, *hi.unwrap()));
                    }
                } else {
                    last = Option::Some((*value, *hi.unwrap()));
                }
            }
        }

        if first.is_some() && last.is_some() {
            let (a, _) = first.unwrap();
            let (b, _) = last.unwrap();

            values.push(format!("{}{}", a, b).parse::<u32>().unwrap());
        }

        first = None;
        last = None;
    }

    let sum: u32 = values.iter().sum();
    println!("The total sum is: {}", sum);

    Ok(())
}
