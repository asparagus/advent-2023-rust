advent_of_code::solution!(8);

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

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
    let left_map: HashMap<&str, &str> = nodes
        .iter()
        .map(|(from, left, _right)| (*from, *left))
        .collect();
    let right_map: HashMap<&str, &str> = nodes
        .iter()
        .map(|(from, _left, right)| (*from, *right))
        .collect();
    let instruction_loop = instructions.chars().cycle();

    let start_node = "AAA";
    let loop_until_goal =
        instruction_loop
            .enumerate()
            .fold_while((start_node, 0), |acc, instruction| {
                let (instruction_count, instruction_char) = instruction;
                let (current, _) = acc;
                let next = match instruction_char {
                    'L' => *left_map.get(current).unwrap(),
                    _ => *right_map.get(current).unwrap(),
                };
                let return_value: (&str, usize) = (next, instruction_count + 1);
                match next {
                    "ZZZ" => Done(return_value),
                    _ => Continue(return_value),
                }
            });
    match loop_until_goal {
        Done((_, count)) => Some(count as u32),
        _ => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
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
    let left_map: HashMap<&str, &str> = nodes
        .iter()
        .map(|(from, left, _right)| (*from, *left))
        .collect();
    let right_map: HashMap<&str, &str> = nodes
        .iter()
        .map(|(from, _left, right)| (*from, *right))
        .collect();

    let nodes_for_start: Vec<&str> = nodes
        .iter()
        .map(|(from, _left, _right)| *from)
        .filter(|node| node.ends_with('A'))
        .collect();
    let mut short_map: HashMap<(&str, usize), (&str, usize)> = HashMap::new();
    // Pile for the nodes / time points we'll explore.
    // We start with the start nodes at t = 0
    let mut pile: Vec<(&str, usize)> = nodes_for_start.iter().map(|&n| (n, 0)).collect();
    let mut seen: HashSet<(&str, usize)> = HashSet::new();
    while let Some((start, offset)) = pile.pop() {
        let (end, steps) = atoz(
            start,
            offset % instructions.len(),
            instructions,
            &left_map,
            &right_map,
        );
        short_map.insert((start, offset), (end, steps));
        let new_offset = (offset + steps) % instructions.len();
        let next_start = (end, new_offset);
        if !seen.contains(&next_start) {
            pile.push(next_start);
            seen.insert(next_start);
        }
    }
    let mut step = 0;
    let mut current: Vec<(&str, usize, usize, &str)> = nodes_for_start
        .iter()
        .map(|&node| {
            let (next_node, num_steps) = *short_map.get(&(node, step)).unwrap();
            (node, step, num_steps, next_node)
        })
        .collect();
    loop {
        let (candidate_nodes, candidate_steps): (Vec<&str>, Vec<usize>) = current
            .iter()
            .map(|&(node, _offset, steps, _next)| (node, steps))
            .unzip();
        let min_step = *candidate_steps.iter().min().unwrap();
        step = min_step;
        if candidate_steps.iter().all_equal()
            && candidate_nodes.iter().all(|node| node.ends_with('Z'))
        {
            break;
        }
        current = current
            .iter()
            .map(|&(node, offset, steps, next)| {
                if steps <= step {
                    let next_offset = (offset + steps) % instructions.len();
                    let (next_next, next_steps) = *short_map.get(&(next, next_offset)).unwrap();
                    (next, next_offset, steps + next_steps, next_next)
                } else {
                    (node, offset, steps, next)
                }
            })
            .collect();
    }
    /*
    This code is actually not efficient enough to reach the answer.
    I feel like there might not be a generic way to efficiently solve this,
    but by checking the generated short_map we can generate the graph and
    observe the solution can easily be computed with the Least common multiple.

    All paths from Ak -> Zk have a corresponding Zk -> Zk loop.
    Both are always the same length.
    The LCM between the corresponding lengths for each starting node yields the answer.
     */
    Some(step as u32)
}

fn atoz<'a>(
    start: &'a str,
    offset: usize,
    instructions: &'a str,
    left_map: &'a HashMap<&'a str, &'a str>,
    right_map: &'a HashMap<&'a str, &'a str>,
) -> (&'a str, usize) {
    let sensible_offset = offset % instructions.len();
    let instruction_loop = instructions.chars().cycle().skip(sensible_offset);
    let loop_until_goal =
        instruction_loop
            .enumerate()
            .fold_while((start, 0), |acc, instruction| {
                let (instruction_count, instruction_char) = instruction;
                let (current, _) = acc;
                let next = match instruction_char {
                    'L' => Some(*left_map.get(current).unwrap()),
                    'R' => Some(*right_map.get(current).unwrap()),
                    _ => None,
                }
                .unwrap();
                let return_value: (&str, usize) = (next, instruction_count + 1);
                match next.ends_with('Z') {
                    true => Done(return_value),
                    false => Continue(return_value),
                }
            });
    let result = match loop_until_goal {
        Done((end, count)) => Some((end, count)),
        _ => None,
    };
    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
