#[derive(Debug)]
pub struct Heat {
    time: i64,
    distance: i64,
}

#[derive(Debug)]
pub struct Data {
    heats: Vec<Heat>,
}

impl TryFrom<&str> for Data {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        fn parse_values(prefix: &str, line: &str) -> Vec<i64> {
            line.get(prefix.len()..)
                .unwrap_or("")
                .split(' ')
                .map(|e| e.trim().parse::<i64>())
                .flatten()
                .collect()
        }

        let lines: Vec<&str> = value.lines().collect();

        let times = lines
            .get(0)
            .map_or(vec![], |line| parse_values("Times:", line));

        let distances = lines
            .get(1)
            .map_or(vec![], |line| parse_values("Distance:", line));

        Ok(Data {
            heats: times
                .iter()
                .zip(distances.iter())
                .map(|(&time, &distance)| Heat { time, distance })
                .collect(),
        })
    }
}

pub fn part_one(data: Data) -> i64 {
    data.heats
        .iter()
        .map(|heat| {
            i64::try_from(
                (0..=heat.time)
                    .map(|charge_up| (charge_up * (heat.time - charge_up)))
                    .filter(|e| e > &heat.distance)
                    .count(),
            )
            .unwrap_or(0)
        })
        .product()
}

pub fn part_two(data: Data) -> i64 {
    let (time, distance) =
        data.heats
            .iter()
            .fold((String::new(), String::new()), |(time, distance), next| {
                (
                    format!("{}{}", time, next.time).to_string(),
                    format!("{}{}", distance, next.distance).to_string(),
                )
            });

    let time: i64 = time.parse().unwrap_or(0);
    let distance: i64 = distance.parse().unwrap_or(0);

    let lower = i64::try_from(
        (0..=(time / 2))
            .map(|charge_up| (charge_up * (time - charge_up)))
            .take_while(|e| e <= &distance)
            .count(),
    )
    .unwrap_or(0);

    let upper = i64::try_from(
        ((time / 2)..=time)
            .rev()
            .map(|charge_up| charge_up * (time - charge_up))
            .take_while(|e| e <= &distance)
            .count(),
    )
    .unwrap_or(0);

    time - (lower + upper) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn part_1_example() -> Result<()> {
        let input = include_str!("../res/06.example");
        assert_eq!(part_one(Data::try_from(input)?), 288);
        Ok(())
    }

    #[test]
    fn part_1_actual() -> Result<()> {
        let input = include_str!("../res/06.actual");
        assert_eq!(part_one(Data::try_from(input)?), 4_568_778);
        Ok(())
    }

    #[test]
    fn part_2_example() -> Result<()> {
        let input = include_str!("../res/06.example");
        assert_eq!(part_two(Data::try_from(input)?), 71_503);
        Ok(())
    }

    #[test]
    fn part_2_actual() -> Result<()> {
        let input = include_str!("../res/06.actual");
        assert_eq!(part_two(Data::try_from(input)?), 28_973_936);
        Ok(())
    }
}
