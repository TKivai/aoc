use std::fmt::Error;

pub fn process_part1(input: &str) -> Result<String, Error> {
    let res = input
        .lines()
        .map(|line| {
            let line_digits = line
                .chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<u32>>();

            if line_digits.len() == 0 {
                0
            } else if line_digits.len() == 1 {
                match line_digits.first() {
                    Some(val) => format!("{:?}{:?}", val, val).parse::<u32>().unwrap(),
                    None => 0,
                }
            } else {
                match (line_digits.first(), line_digits.last()) {
                    (Some(first), Some(last)) => {
                        format!("{:?}{:?}", first, last).parse::<u32>().unwrap()
                    }
                    _ => 0,
                }
            }
        })
        .reduce(|acc, val| acc + val);

    match res {
        Some(final_result) => Ok(final_result.to_string()),
        None => Err(Error),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> Result<(), Error> {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(process_part1(input)?, "142".to_string());
        Ok(())
    }
}
