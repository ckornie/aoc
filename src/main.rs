use std::io::Error;

fn main() -> Result<(), Error> {
    let data = include_str!("../resources/2023-12-01-001.txt");
    for line in data.split('\n') {
        println!("{}", line);
    }

    Ok(())
}
