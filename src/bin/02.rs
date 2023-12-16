advent_of_code::solution!(2);

use regex::Regex;

fn validate(color: &str, number: u32) -> bool {
    match color {
        "red" => number <= 12,
        "green" => number <= 13,
        "blue" => number <= 14,
        _ => false,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let line_regex = Regex::new(r"Game (?<id>\d+): (?<game>.*)").unwrap();
    let cube_regex = Regex::new(r"(?<number>\d+) (?<color>[[:alpha:]]+)").unwrap();
    let lines = input.split('\n');
    let non_empty_lines = lines.filter(|line| !line.is_empty());
    let ids_and_games = non_empty_lines.map(|line| {
        let caps = line_regex.captures(line).unwrap();
        (
            caps.name("id").unwrap().as_str(),
            caps.name("game").unwrap().as_str(),
        )
    });
    let possible_ids_and_games = ids_and_games.filter(|(_id, game)| {
        cube_regex.captures_iter(game).all(|cap| {
            validate(
                cap.name("color").unwrap().as_str(),
                cap.name("number").unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
    });
    let possible_ids_parsed = possible_ids_and_games.map(|(id, _game)| id.parse::<u32>().unwrap());
    possible_ids_parsed.reduce(|acc, e| acc + e)
    // Game 1: 1 red, 3 blue, 11 green; 1 blue, 5 red; 3 blue, 5 green, 13 red; 6 red, 1 blue, 4 green; 16 red, 12 green
}

pub fn part_two(input: &str) -> Option<u32> {
    let line_regex = Regex::new(r"Game (?<id>\d+): (?<game>.*)").unwrap();
    let red_regex = Regex::new(r"(?<number>\d+) red").unwrap();
    let blue_regex = Regex::new(r"(?<number>\d+) blue").unwrap();
    let green_regex = Regex::new(r"(?<number>\d+) green").unwrap();
    let lines = input.split('\n');
    let non_empty_lines = lines.filter(|line| !line.is_empty());
    let ids_and_games = non_empty_lines.map(|line| {
        let caps = line_regex.captures(line).unwrap();
        (
            caps.name("id").unwrap().as_str(),
            caps.name("game").unwrap().as_str(),
        )
    });
    let game_powers = ids_and_games.map(|(_id, game)| {
        red_regex
            .captures_iter(game)
            .map(|caps| {
                caps.name("number")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .unwrap()
            })
            .max()
            .unwrap_or(0)
            * blue_regex
                .captures_iter(game)
                .map(|caps| {
                    caps.name("number")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap()
                })
                .max()
                .unwrap_or(0)
            * green_regex
                .captures_iter(game)
                .map(|caps| {
                    caps.name("number")
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap()
                })
                .max()
                .unwrap_or(0)
    });
    game_powers.reduce(|acc, e| acc + e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8u32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286u32));
    }
}
