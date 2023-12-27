advent_of_code::solution!(12);

use itertools::Itertools;
use regex::Regex;

struct ProblemInput {
    condition_records: String,
    failure_groups: Vec<usize>,
}

fn parse_line(condition_records_regex: &Regex, numbers_regex: &Regex, line: &str) -> ProblemInput {
    let failure_groups = numbers_regex
        .find_iter(line)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect();
    let condition_records = condition_records_regex.find(line).unwrap().as_str();
    ProblemInput {
        condition_records: condition_records.to_string(),
        failure_groups,
    }
}

fn plausible_starts(condition_records: &str, failure_group: usize) -> Vec<usize> {
    (0..condition_records.len())
        .map(|start| (start, start + failure_group))
        .filter(|&(_start, end)| end <= condition_records.len())
        .filter(|&(_start, end)| {
            end == condition_records.len() || !condition_records[end..].starts_with('#')
        })
        .filter(|&(start, _end)| start == 0 || !condition_records[..start].ends_with('#'))
        .filter(|&(start, end)| {
            condition_records[start..end]
                .chars()
                .all(|c| c == '#' || c == '?')
        })
        .map(|(start, _end)| start)
        .collect()
}

fn count_arrangements(problem_input: &ProblemInput, visible_failures_regex: &Regex) -> u32 {
    let mut plausible_failure_group_arrangements: Vec<Vec<usize>> = problem_input
        .failure_groups
        .iter()
        .map(|&failure_group| plausible_starts(&problem_input.condition_records, failure_group))
        .collect();
    let visible_failure_groups: Vec<(usize, usize)> = visible_failures_regex
        .find_iter(&problem_input.condition_records)
        .map(|m| (m.start(), m.len()))
        .collect();
    if !plausible_failure_group_arrangements.is_empty() {
        if !visible_failure_groups.is_empty() {
            // Restrict based on first visible failure
            let (first_start, _first_len) = visible_failure_groups.first().unwrap();
            let first_failure_group_plausible_starts =
                plausible_failure_group_arrangements.first_mut().unwrap();
            // Starts later than the first visible failure are not plausible
            first_failure_group_plausible_starts.retain(|s| s <= first_start);
            // Restrict based on last visible failure
            let (last_start, last_len) = visible_failure_groups.last().unwrap();
            let last_index = last_start + last_len - 1;
            let last_failure_group_plausible_starts =
                plausible_failure_group_arrangements.last_mut().unwrap();
            // Starts earlier than would include the last visible failure are not plausible
            let last_failure_group = problem_input.failure_groups.last().unwrap();
            last_failure_group_plausible_starts.retain(|s| s + last_failure_group >= last_index);
        }
        // Restrict based on failure group arrangements ordering
        // Each group cannot be earlier than the previous group could start and finish
        (0..plausible_failure_group_arrangements.len())
            .tuple_windows::<(_, _)>()
            .for_each(|(prev_idx, next_idx)| {
                let previous_len = problem_input.failure_groups[prev_idx];
                let previous_min_start = *plausible_failure_group_arrangements[prev_idx]
                    .iter()
                    .min()
                    .unwrap();
                plausible_failure_group_arrangements[next_idx]
                    .retain(|&s| s > previous_min_start + previous_len);
            });
        // Restrict based on failure group arrangements ordering
        // Each group cannot be later than to finish before the next group starts
        (0..plausible_failure_group_arrangements.len())
            .rev()
            .tuple_windows::<(_, _)>()
            .for_each(|(next_idx, prev_idx)| {
                let previous_len = problem_input.failure_groups[prev_idx];
                let next_max_start = *plausible_failure_group_arrangements[next_idx]
                    .iter()
                    .max()
                    .unwrap();
                plausible_failure_group_arrangements[prev_idx]
                    .retain(|&s| s + previous_len < next_max_start);
            });
    }
    if plausible_failure_group_arrangements.len() <= 1 {
        plausible_failure_group_arrangements
            .first()
            .unwrap_or(&vec![])
            .len() as u32
    } else {
        validate_combinations(
            &problem_input.condition_records,
            &problem_input.failure_groups,
            &plausible_failure_group_arrangements,
            0,
        )
    }
}

fn validate_combinations(
    condition_records: &str,
    failure_groups: &[usize],
    plausible_failure_group_arrangements: &[Vec<usize>],
    offset: usize,
) -> u32 {
    if let (
        Some((first_plausible_failure_group_arrangement, rest_plausible_failure_group_arrangement)),
        Some((first_failure_group, rest_failure_groups)),
    ) = (
        plausible_failure_group_arrangements.split_first(),
        failure_groups.split_first(),
    ) {
        let min_start = match offset {
            0 => 0,
            value => value + 1,
        };
        let max_start = offset + condition_records.find('#').unwrap_or(usize::MAX - offset);
        first_plausible_failure_group_arrangement
            .iter()
            .filter(|&&start| start >= min_start)
            .filter(|&&start| start <= max_start)
            .map(|&start| {
                let end = start + first_failure_group;
                validate_combinations(
                    &condition_records[end - offset..],
                    rest_failure_groups,
                    rest_plausible_failure_group_arrangement,
                    end,
                )
            })
            .sum()
    } else {
        1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers_regex = Regex::new(r"\d+").unwrap();
    let condition_records_regex = Regex::new(r"[\.#\?]+").unwrap();
    let visible_failures_regex = Regex::new(r"#+").unwrap();
    let parse = |line| parse_line(&condition_records_regex, &numbers_regex, line);
    let inputs: Vec<ProblemInput> = input.trim().split('\n').map(parse).collect();
    let result = inputs
        .iter()
        .map(|problem_input| count_arrangements(problem_input, &visible_failures_regex))
        .sum();
    Some(result)
}

fn unfold(problem_input: ProblemInput) -> (String, Vec<usize>) {
    let copies = [
        &problem_input.condition_records[..],
        &problem_input.condition_records[..],
        &problem_input.condition_records[..],
        &problem_input.condition_records[..],
        &problem_input.condition_records[..],
    ];
    let unfolded_condition_records = copies.join("?");
    let unfolded_failure_groups = problem_input.failure_groups.repeat(5);
    (unfolded_condition_records, unfolded_failure_groups)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers_regex = Regex::new(r"\d+").unwrap();
    let condition_records_regex = Regex::new(r"[\.#\?]+").unwrap();
    let visible_failures_regex = Regex::new(r"#+").unwrap();
    let parse = |line| parse_line(&condition_records_regex, &numbers_regex, line);
    let inputs: Vec<ProblemInput> = input
        .trim()
        .split('\n')
        .map(parse)
        .map(unfold)
        .map(|(condition_records, failure_groups)| ProblemInput {
            condition_records: condition_records.to_string(),
            failure_groups,
        })
        .collect();
    let result = inputs
        .iter()
        .map(|problem_input| count_arrangements(problem_input, &visible_failures_regex))
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
