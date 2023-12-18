advent_of_code::solution!(5);

use regex::Regex;

fn parse(input: &str) -> (Vec<u32>, Vec<Vec<(u32, u32, u32)>>) {
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
    let maps: Vec<Vec<(u32, u32, u32)>> = map_info
        .map(|m| {
            m.trim()
                .split('\n')
                .map(|s| {
                    let cap = map_numbers_regex.captures(s).unwrap();
                    (
                        cap.name("idx_to").unwrap().as_str().parse::<u32>().unwrap(),
                        cap.name("idx_from")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap(),
                        cap.name("len").unwrap().as_str().parse::<u32>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();
    (seeds, maps)
}

fn apply_mapping(current: u32, mapping: &Vec<(u32, u32, u32)>) -> u32 {
    let retrieved_result = mapping
        .iter()
        .filter_map(|(idx_to, idx_from, len)| {
            match (*idx_from..(*idx_from + *len)).contains(&current) {
                true => Some(idx_to + (current - idx_from)),
                false => None,
            }
        })
        .next();
    retrieved_result.unwrap_or(current)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse(input);
    // mapped_seeds
    seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, apply_mapping))
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
