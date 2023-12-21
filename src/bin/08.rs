advent_of_code::solution!(8);

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let instructions_re = Regex::new(r"^[LR]+").unwrap();
    let instructions = instructions_re.find(input).unwrap().as_str();
    let nodes_re = Regex::new(r"(?<from>\w{3}) = \((?<L>\w{3}), (?<R>\w{3})\)").unwrap();
    let nodes: Vec<(&str, &str, &str)> = nodes_re
        .captures_iter(input)
        .map(|c| {
            (
                c.name("from").unwrap().as_str(),
                c.name("L").unwrap().as_str(),
                c.name("R").unwrap().as_str(),
            )
        })
        .collect();
    let left_map: HashMap<&&str, &&str> = nodes
        .iter()
        .map(|(from, left, _right)| (from, left))
        .collect();
    let right_map: HashMap<&&str, &&str> = nodes
        .iter()
        .map(|(from, _left, right)| (from, right))
        .collect();
    let instruction_loop = instructions.chars().cycle();

    let start_node = &"AAA";
    let loop_until_goal =
        instruction_loop
            .enumerate()
            .fold_while((start_node, 0), |acc, instruction| {
                let (instruction_count, instruction_char) = instruction;
                let (current, _) = acc;
                let next = match instruction_char {
                    'L' => left_map.get(current),
                    'R' => right_map.get(current),
                    _ => None,
                };
                if let Some(value) = next {
                    let return_value: (&&str, usize) = (value, instruction_count + 1);
                    match value {
                        &&"ZZZ" => Done(return_value),
                        _ => Continue(return_value)
                    }
                } else {
                    Done((current, instruction_count))
                }
            });
    match loop_until_goal {
        Done((_, count)) => Some(count as u32),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
