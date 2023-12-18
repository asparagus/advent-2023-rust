advent_of_code::solution!(5);

use regex::Regex;
use std::str::FromStr;
struct MappingSlice {
    index_from: u32,
    index_to: u32,
    len: u32,
}

fn retrieve_unwrap_parse<T: FromStr>(captures: &regex::Captures, name: &str) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    captures.name(name).unwrap().as_str().parse::<T>().unwrap()
}

fn parse(input: &str) -> (Vec<u32>, Vec<Vec<MappingSlice>>) {
    let seeds_regex = Regex::new(r"seeds: .*\n").unwrap();
    let map_regex =
        Regex::new(r"(?<map_declaration>\S+ map:)\n(?<map_elements>(?:\d+ \d+ \d+\n)+)").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let map_numbers_regex = Regex::new(r"(?<idx_to>\d+) (?<idx_from>\d+) (?<len>\d+)").unwrap();

    let seeds_info = seeds_regex.find(input).unwrap().as_str().trim();
    let seeds = number_regex
        .find_iter(seeds_info)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();

    let map_info = map_regex
        .captures_iter(input)
        .map(|m| m.name("map_elements").unwrap().as_str());
    let maps: Vec<Vec<MappingSlice>> = map_info
        .map(|m| {
            m.trim()
                .split('\n')
                .map(|s| {
                    let cap = map_numbers_regex.captures(s).unwrap();
                    MappingSlice {
                        index_from: retrieve_unwrap_parse::<u32>(&cap, "idx_from"),
                        index_to: retrieve_unwrap_parse::<u32>(&cap, "idx_to"),
                        len: retrieve_unwrap_parse::<u32>(&cap, "len"),
                    }
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}

fn try_apply_mapping_slice(current: u32, mapping_slice: &MappingSlice) -> Option<u32> {
    let start = mapping_slice.index_from;
    let end = start.checked_add(mapping_slice.len - 1).unwrap_or(u32::MAX);
    let range = start..=end;
    match range.contains(&current) {
        true => Some(current - mapping_slice.index_from + mapping_slice.index_to),
        _ => None,
    }
}

fn apply_mapping(current: u32, mapping: &[MappingSlice]) -> u32 {
    let retrieved_result = mapping
        .iter()
        .filter_map(|mapping_slice| try_apply_mapping_slice(current, mapping_slice))
        .next();
    retrieved_result.unwrap_or(current)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse(input);
    // mapped_seeds
    seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |current, mapping| {
                apply_mapping(current, &mapping[..])
            })
        })
        .min()
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
