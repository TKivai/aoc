use std::fmt::Error;

pub fn process_part2(input: &str) -> Result<String, Error> {
    let res = input
        .lines()
        .map(|line| {
            let line_digits = nums_in_line(line);

            if line_digits.len() == 0 {
                0
            } else if line_digits.len() == 1 {
                match line_digits.first() {
                    Some(val) => (val * 10) + val,
                    None => 0,
                }
            } else {
                match (line_digits.first(), line_digits.last()) {
                    (Some(first), Some(last)) => (first * 10) + last,
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

fn add_nums_from_spelling(line: &str, line_pos: &usize, line_vec: &mut Vec<u32>) {
    let numbers = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    for n in 0..numbers.len() {
        match line.split_at(*line_pos).1.starts_with(numbers[n].0) {
            true => {
                line_vec.push(numbers[n].1);
            }
            false => {}
        };
    }
}

fn nums_in_line(line: &str) -> Vec<u32> {
    let mut final_vec: Vec<u32> = vec![];
    line.chars()
        .into_iter()
        .enumerate()
        .for_each(|(index, curr_char)| match curr_char.to_digit(10) {
            Some(digit) => {
                final_vec.push(digit);
            }
            None => add_nums_from_spelling(line, &index, &mut final_vec),
        });
    final_vec
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process() -> Result<(), Error> {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(process_part2(input)?, "281".to_string());

        Ok(())
    }

    #[test]
    fn test_num_from_alphabet() -> Result<(), Error> {
        let input1 = "4eight5mjlkzrgnmlnmxntqmtlxmqlkjccttcpmgznfouroneightk";
        let input2 = "zoneight234";
        let input3 = "7pqrstsixteen";
        let input4 = "mbvtbcjvv33rqfsllshb";

        assert_eq!(nums_in_line(input1), vec![4, 8, 5, 4, 1, 8]);
        assert_eq!(nums_in_line(input2), vec![1, 8, 2, 3, 4]);
        assert_eq!(nums_in_line(input3), vec![7, 6]);
        assert_eq!(nums_in_line(input4), vec![3, 3]);
        Ok(())
    }
}
