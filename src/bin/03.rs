advent_of_code::solution!(3);

use itertools::iproduct;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let symbols_regex = Regex::new(r"[^\d\.]").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_locations = input
        .split('\n')
        .enumerate()
        .flat_map(|(row, line)| symbols_regex.find_iter(line).map(move |m| (row, m.start())));
    let _collected_symbols: HashSet<(usize, usize)> = symbol_locations.collect();
    let numbers = input.split('\n').enumerate().flat_map(|(row, line)| {
        number_regex.find_iter(line).map(move |m| {
            let value = m.as_str().parse::<u32>().unwrap();
            (row, m.start(), m.end(), value)
        })
    });
    let valid_numbers = numbers.filter_map(|(row, start, end, value)| {
        let row_start = match row {
            0 => 0,
            _ => row - 1,
        };
        let col_start = match start {
            0 => 0,
            _ => start - 1,
        };
        for (r, c) in iproduct!(row_start..row + 2, col_start..end + 1) {
            if _collected_symbols.contains(&(r, c)) {
                return Some(value);
            }
        }
        None
    });
    valid_numbers.reduce(|acc, e| acc + e)
}

pub fn part_two(input: &str) -> Option<u32> {
    let symbols_regex = Regex::new(r"[^\d\.]").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let symbol_locations = input
        .split('\n')
        .enumerate()
        .flat_map(|(row, line)| symbols_regex.find_iter(line).map(move |m| (row, m.start())));
    let _collected_symbols: HashSet<(usize, usize)> = symbol_locations.collect();
    let numbers = input.split('\n').enumerate().flat_map(|(row, line)| {
        number_regex.find_iter(line).map(move |m| {
            let value = m.as_str().parse::<u32>().unwrap();
            (row, m.start(), m.end(), value)
        })
    });
    let mut locations_to_numbers = HashMap::new();
    for (row, start, end, value) in numbers {
        let row_start = match row {
            0 => 0,
            _ => row - 1,
        };
        let col_start = match start {
            0 => 0,
            _ => start - 1,
        };
        for (r, c) in iproduct!(row_start..row + 2, col_start..end + 1) {
            locations_to_numbers
                .entry((r, c))
                .or_insert_with(Vec::<u32>::new);
            locations_to_numbers.get_mut(&(r, c)).unwrap().push(value);
        }
    }
    let mut result = 0u32;
    for ((row, col), group) in locations_to_numbers {
        if _collected_symbols.contains(&(row, col)) && group.len() == 2 {
            result += group.into_iter().product::<u32>();
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
