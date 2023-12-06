use std::io::Error;

fn main() -> Result<(), Error> {
    let input = include_str!("../../res/01.txt");
    let mut line: Vec<char> = Vec::new();
    let mut values: Vec<u32> = Vec::new();

    for c in input.chars() {
        if c.is_digit(10) {
            line.push(c);
        } else if c == '\n' {
            if line.len() >= 1 {
                let first = line.first().unwrap();
                let second = line.last().unwrap();
                let mut number = first.to_string();
                number.push(*second);
                values.push(number.parse::<u32>().unwrap());
            } else {
                eprintln!("Did not find any digits.");
            }

            line.clear();
        }
    }

    let sum: u32 = values.iter().sum();
    println!("The total sum is: {}", sum);

    Ok(())
}
