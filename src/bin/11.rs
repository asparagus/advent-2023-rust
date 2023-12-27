use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let galaxy_locations: Vec<(usize, usize)> = input
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(move |(row_idx, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(col_idx, cell)| match cell {
                    '#' => Some((row_idx, col_idx)),
                    '.' => None,
                    _ => unimplemented!(),
                })
        })
        .collect();
    let rows_with_galaxies = galaxy_locations
        .iter()
        .map(|&(row_idx, _col_idx)| row_idx)
        .collect::<HashSet<usize>>();
    let cols_with_galaxies = galaxy_locations
        .iter()
        .map(|&(_row_idx, col_idx)| col_idx)
        .collect::<HashSet<usize>>();
    let sum_distances: u64 = galaxy_locations
        .iter()
        .combinations(2)
        .map(|pair| {
            space_dilated_distance(
                pair.get(0).unwrap(),
                pair.get(1).unwrap(),
                &rows_with_galaxies,
                &cols_with_galaxies,
                2,
            )
        })
        .sum();
    Some(sum_distances)
}

fn space_dilated_distance(
    first: &(usize, usize),
    second: &(usize, usize),
    rows_with_galaxies: &HashSet<usize>,
    cols_with_galaxies: &HashSet<usize>,
    dilation_factor: u64,
) -> u64 {
    let &(first_row, first_col) = first;
    let &(second_row, second_col) = second;
    let abs_diff = first_row.abs_diff(second_row) + first_col.abs_diff(second_col);
    let dilated_rows = (first_row..second_row)
        .filter(|row| !rows_with_galaxies.contains(row))
        .count()
        + (second_row..first_row)
            .filter(|row| !rows_with_galaxies.contains(row))
            .count();
    let dilated_cols = (first_col..second_col)
        .filter(|col| !cols_with_galaxies.contains(col))
        .count()
        + (second_col..first_col)
            .filter(|col| !cols_with_galaxies.contains(col))
            .count();
    abs_diff as u64 + (dilated_rows + dilated_cols) as u64 * (dilation_factor - 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let galaxy_locations: Vec<(usize, usize)> = input
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(move |(row_idx, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(col_idx, cell)| match cell {
                    '#' => Some((row_idx, col_idx)),
                    '.' => None,
                    _ => unimplemented!(),
                })
        })
        .collect();
    let rows_with_galaxies = galaxy_locations
        .iter()
        .map(|&(row_idx, _col_idx)| row_idx)
        .collect::<HashSet<usize>>();
    let cols_with_galaxies = galaxy_locations
        .iter()
        .map(|&(_row_idx, col_idx)| col_idx)
        .collect::<HashSet<usize>>();
    let sum_distances: u64 = galaxy_locations
        .iter()
        .combinations(2)
        .map(|pair| {
            space_dilated_distance(
                pair.get(0).unwrap(),
                pair.get(1).unwrap(),
                &rows_with_galaxies,
                &cols_with_galaxies,
                1000000,
            )
        })
        .sum();
    Some(sum_distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
