advent_of_code::solution!(4);

use regex::Regex;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"Card\s+\d+: (?<winning_numbers>[\d\s]+)\|(?<numbers_you_have>[\d\s]+)")
        .unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let lines = input.trim().split('\n');
    let captures = lines.map(|line| re.captures(line).unwrap());
    let num_winners = captures.map(|cap| {
        count_winners(
            &number_regex,
            cap.name("winning_numbers").unwrap().as_str(),
            cap.name("numbers_you_have").unwrap().as_str(),
        )
    });
    num_winners
        .map(score_num_winners)
        .reduce(|acc, e| (acc + e))
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"Card\s+\d+: (?<winning_numbers>[\d\s]+)\|(?<numbers_you_have>[\d\s]+)")
        .unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let lines = input.trim().split('\n');
    let captures = lines.map(|line| re.captures(line).unwrap());
    let num_winners = captures.map(|cap| {
        count_winners(
            &number_regex,
            cap.name("winning_numbers").unwrap().as_str(),
            cap.name("numbers_you_have").unwrap().as_str(),
        )
    });
    let all_num_winners: Vec<u32> = num_winners.collect();
    let mut repeats: Vec<u32> = vec![1; all_num_winners.len()];
    for (idx, winners) in all_num_winners.iter().enumerate() {
        let top_cap: usize = std::cmp::min(
            idx + 1 + usize::try_from(*winners).unwrap(),
            all_num_winners.len(),
        );
        for other_idx in (idx + 1)..top_cap {
            repeats[other_idx] += repeats[idx];
        }
    }
    repeats.into_iter().reduce(|acc, e| (acc + e))
}

fn count_winners(number_regex: &Regex, winning_numbers: &str, numbers_you_have: &str) -> u32 {
    let winning: HashSet<&str> = number_regex
        .find_iter(winning_numbers)
        .map(|m| m.as_str())
        .collect();
    let winning_numbers_you_have = number_regex
        .find_iter(numbers_you_have)
        .filter(|m| winning.contains(m.as_str()));
    winning_numbers_you_have.count().try_into().unwrap()
}

fn score_num_winners(num_winners: u32) -> u32 {
    match num_winners {
        0 => 0,
        v => 2u32.pow(v - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
