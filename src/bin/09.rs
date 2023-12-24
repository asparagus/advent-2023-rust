advent_of_code::solution!(9);

use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let numbers_re = Regex::new(r"-?\d+").unwrap();
    let lines = input.trim().split('\n');
    let number_matches = lines
        .map(|line| {
            numbers_re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let results: Vec<i32> = number_matches
        .iter()
        .map(|numbers| extrapolate(numbers.iter()))
        .collect();
    Some(results.iter().sum::<i32>())
}

fn extrapolate(vals: std::slice::Iter<'_, i32>) -> i32 {
    // Constant extrapolation
    // vals.fold(None, |_acc, &e| Some(e)).unwrap_or_default()

    // First order extrapolation
    // vals.fold((0, (0, 0)), |(extrapolation, derivatives), &e| {
    //     let new_derivatives = (derivatives.0, e + derivatives.1 - extrapolation);
    //     let new_extrapolation = e + new_derivatives.1;
    //     (new_extrapolation, new_derivatives)
    // })
    // .0

    // Second order extrapolation
    // vals.fold((0, (0, 0, 0)), |(_extrapolation, derivatives), &e| {
    //     let new_derivatives = (e, e - derivatives.0, e - derivatives.0 - derivatives.1);
    //     let new_extrapolation = e + new_derivatives.1;
    //     (new_extrapolation, new_derivatives)
    // })
    // .0

    // Arbitrary order extrapolation
    vals.fold((0, vec![]), |(_extrapolation, derivatives), &e| {
        let mut new_derivatives = vec![e; derivatives.len() + 1];
        let mut acc = 0;
        derivatives.iter().enumerate().for_each(|(idx, d)| {
            acc += d;
            *new_derivatives.get_mut(idx + 1).unwrap() -= acc;
        });
        let new_extrapolation = new_derivatives.iter().sum();
        (new_extrapolation, new_derivatives)
    })
    .0
}

pub fn part_two(input: &str) -> Option<i32> {
    let numbers_re = Regex::new(r"-?\d+").unwrap();
    let lines = input.trim().split('\n');
    let number_matches = lines
        .map(|line| {
            numbers_re
                .find_iter(line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let results: Vec<i32> = number_matches
        .iter()
        .map(|numbers| extrapolate_backwards(numbers.iter()))
        .collect();
    Some(results.iter().sum::<i32>())
}

fn extrapolate_backwards(vals: std::slice::Iter<'_, i32>) -> i32 {
    vals.fold((0, vec![]), |(extrapolation, derivatives), &e| {
        let mut new_derivatives = vec![e; derivatives.len() + 1];
        let mut acc = 0;
        derivatives.iter().enumerate().for_each(|(idx, d)| {
            acc += d;
            *new_derivatives.get_mut(idx + 1).unwrap() -= acc;
        });
        let new_extrapolation = extrapolation
            + new_derivatives.last().unwrap() * (-1i32).pow(new_derivatives.len() as u32 + 1);
        (new_extrapolation, new_derivatives)
    })
    .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
