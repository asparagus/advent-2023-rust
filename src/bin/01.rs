advent_of_code::solution!(1);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let non_empty_lines = lines.filter(|line| !line.is_empty());
    let digits = non_empty_lines.map(|line| digits(line));
    let values = digits.map(
        |(first, last)|
        first * 10u32 + last
    );
    values.reduce(|acc, e| acc + e)
}

pub fn digits(input: &str) -> (u32, u32) {
    let first = input.chars().filter(
        char::is_ascii_digit).map(|c| c.to_digit(10).unwrap()).next();
    let last = input.chars().rev().filter(
        char::is_ascii_digit).map(
            |c| c.to_digit(10).unwrap()
        ).next();
    (first.unwrap(), last.unwrap())
}

pub fn part_two(_input: &str) -> Option<u32> {
    let first_digit_regex = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine|zero).*?").unwrap();
    let last_digit_regex = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine|zero)").unwrap();
    let lines = _input.split("\n");
    let non_empty_lines = lines.filter(|line| !line.is_empty());
    let digits = non_empty_lines.map(
        |line|
        (
            first_digit_regex.captures(line).unwrap().get(1).unwrap().as_str(),
            last_digit_regex.captures(line).unwrap().get(1).unwrap().as_str()
        )
    );
    let values = digits.map(
        |(first, last)|
        parse_digit_from_str(first) * 10u32 + parse_digit_from_str(last)
    );
    values.reduce(|acc, e| acc + e)
}

pub fn parse_digit_from_str(_input: &str) -> u32 {
    match _input.len() {
        1 => _input.chars().next().unwrap().to_digit(10).unwrap(),
        _ => match _input {
            "one" => 1u32,
            "two" => 2u32,
            "three" => 3u32,
            "four" => 4u32,
            "five" => 5u32,
            "six" => 6u32,
            "seven" => 7u32,
            "eight" => 8u32,
            "nine" => 9u32,
            "zero" => 0u32,
            _ => panic!("Not implemented {_input}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142u32));
    }
}
