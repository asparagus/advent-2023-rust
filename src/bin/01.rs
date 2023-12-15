advent_of_code::solution!(1);

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
    None
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
        assert_eq!(result, None);
    }
}
