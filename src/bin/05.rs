advent_of_code::solution!(5);

use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::ops::Range;
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
    let (seed_inputs, maps) = parse(input);
    let (seed_starts, seed_lengths): (_, Vec<_>) = seed_inputs
        .iter()
        .enumerate()
        .partition(|(idx, _val)| (idx % 2) == 0);
    // Modify the previously read into seed ranges
    let seed_ranges: Vec<Range<u32>> = seed_starts
        .iter()
        .zip(seed_lengths.iter())
        .map(|((_, start), (_, length))|
                // TODO: ADDRESS OVERFLOW
                (**start)..(**start + **length))
        .collect();
    // Pass each seed range through all the mappings
    let mapped_seeds: Vec<Range<u32>> = seed_ranges
        .iter()
        .flat_map(|seed_range| {
            #[allow(clippy::single_range_in_vec_init)]
            let start = vec![seed_range.start..seed_range.end];
            maps.iter().fold(start, |ranges, mapping| {
                apply_range_mapping(&ranges, mapping)
            })
        })
        .collect();
    let min = mapped_seeds.iter().map(|range| range.start).min();
    min
}

fn apply_range_mapping(ranges: &[Range<u32>], mapping: &[MappingSlice]) -> Vec<Range<u32>> {
    ranges
        .iter()
        .flat_map(|range| {
            #[allow(clippy::single_range_in_vec_init)]
            let start = vec![range.start..range.end];
            let mut leftovers = mapping.iter().fold(start, |remainder, slice| {
                remainder
                    .iter()
                    .flat_map(|remainder_range| {
                        let slice_start = slice.index_from;
                        let slice_end = slice_start.checked_add(slice.len).unwrap_or(u32::MAX);
                        let beneath: Range<u32> =
                            remainder_range.start..min(slice_start, remainder_range.end - 1);
                        let above: Range<u32> =
                            (max(slice_end, remainder_range.start))..remainder_range.end;

                        if !beneath.is_empty() && !above.is_empty() {
                            vec![beneath, above]
                        } else if !beneath.is_empty() {
                            vec![beneath]
                        } else if !above.is_empty() {
                            vec![above]
                        } else {
                            vec![]
                        }
                    })
                    .collect()
            });
            let mut mapped: Vec<Range<u32>> = mapping
                .iter()
                .filter_map(|slice| {
                    let slice_start = slice.index_from;
                    let slice_end = slice_start.checked_add(slice.len - 1).unwrap_or(u32::MAX);
                    let intersection: Range<u32> =
                        max(slice_start, range.start)..min(slice_end, range.end);
                    if intersection.is_empty() {
                        None
                    } else if slice.index_to >= slice.index_from {
                        let offset = slice.index_to - slice.index_from;
                        Some((offset + intersection.start)..(offset + intersection.end))
                    } else {
                        let negative_offset = slice.index_from - slice.index_to;
                        Some(
                            (intersection.start - negative_offset)
                                ..(intersection.end - negative_offset),
                        )
                    }
                })
                .collect();
            leftovers.append(&mut mapped);
            leftovers
        })
        .collect()
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
        assert_eq!(result, Some(46));
    }
}
