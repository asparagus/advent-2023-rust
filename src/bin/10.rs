use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim().split('\n');
    let tiles: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let start = tiles
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, cell)| match cell {
                    'S' => Some((row_idx, col_idx)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();
    let result = navigate(&tiles, start);
    Some(result)
}

fn navigate(tiles: &Vec<Vec<char>>, start: (usize, usize)) -> u32 {
    let (height, width) = (tiles.len(), tiles.get(0).unwrap().len());
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert(start);

    let mut current_nodes: Vec<(u32, (usize, usize))> = vec![];
    let mut next_nodes = vec![(0, start)];
    while !next_nodes.is_empty() {
        current_nodes = next_nodes;
        next_nodes = current_nodes
            .iter()
            .flat_map(|&(steps, node)| {
                let (row_index, col_index) = node;
                let tile = tiles.get(row_index).unwrap().get(col_index).unwrap();
                let neighbor_candidates = expand(tile, node);
                let validated_neighbors: Vec<(u32, (usize, usize))> = neighbor_candidates
                    .iter()
                    .filter(|candidate| !seen.contains(candidate))
                    .filter(|(row_index, col_index)| {
                        (0..height).contains(row_index) && (0..width).contains(col_index)
                    })
                    .map(|&(row_index, col_index)| {
                        // Valid position
                        let tile = tiles.get(row_index).unwrap().get(col_index).unwrap();
                        (tile, (row_index, col_index))
                    })
                    .filter(|&(tile, candidate)| expand(tile, candidate).contains(&node))
                    .map(|(_tile, candidate)| (steps + 1, candidate))
                    .collect();
                validated_neighbors
            })
            .collect();
        next_nodes.iter().for_each(|&(_, node)| {
            seen.insert(node);
        });
    }
    let path_lengths = current_nodes
        .iter()
        .map(|&(step, _node)| step)
        .max()
        .unwrap_or_default();
    path_lengths
}

fn expand(tile: &char, position: (usize, usize)) -> Vec<(usize, usize)> {
    let (row, col) = (position.0 as i32, position.1 as i32);
    match tile {
        '.' => Some(vec![]),
        '|' => Some(vec![(row - 1, col), (row + 1, col)]),
        '-' => Some(vec![(row, col - 1), (row, col + 1)]),
        'L' => Some(vec![(row - 1, col), (row, col + 1)]),
        'J' => Some(vec![(row - 1, col), (row, col - 1)]),
        '7' => Some(vec![(row, col - 1), (row + 1, col)]),
        'F' => Some(vec![(row + 1, col), (row, col + 1)]),
        'S' => Some(vec![
            (row - 1, col),
            (row + 1, col),
            (row, col - 1),
            (row, col + 1),
        ]),
        _ => None,
    }
    .unwrap()
    .iter()
    .filter(|&&(r, c)| r >= 0 && c >= 0)
    .map(|&(r, c)| (r as usize, c as usize))
    .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().split('\n');
    let mut tiles: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let start = tiles
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(col_idx, cell)| match cell {
                    'S' => Some((row_idx, col_idx)),
                    _ => None,
                })
                .next()
        })
        .next()
        .unwrap();
    let start_tile_content = infer_start_tile(&tiles, start);
    let start_cell = tiles.get_mut(start.0).unwrap().get_mut(start.1).unwrap();
    *start_cell = start_tile_content;
    let loop_nodes = identify_loop(&tiles, start);
    let area = count_area(&tiles, &loop_nodes);
    Some(area)
}

fn infer_start_tile(tiles: &[Vec<char>], start: (usize, usize)) -> char {
    let neighbors = expand(&'S', start);
    let matches: ((usize, usize), (usize, usize)) = neighbors
        .iter()
        .map(|&(row_index, col_index)| {
            // Valid position
            let tile = tiles.get(row_index).unwrap().get(col_index).unwrap();
            (tile, (row_index, col_index))
        })
        .filter(|&(tile, candidate)| expand(tile, candidate).contains(&start))
        .map(|(_tile, candidate)| candidate)
        .collect_tuple()
        .unwrap();
    let possibilities = ['|', '-', 'L', 'J', '7', 'F'];
    *possibilities
        .iter()
        .find(|&tile| {
            let expansions_to_compare = expand(tile, start);
            expansions_to_compare.len() == 2
                && expansions_to_compare
                    .iter()
                    .all(|&n| n == matches.0 || n == matches.1)
        })
        .unwrap()
}

enum BoundaryStatus {
    Outside,
    Inside,
    EnterFromAbove,
    EnterFromBelow,
    ExitFromAbove,
    ExitFromBelow,
}

fn count_area(tiles: &[Vec<char>], loop_nodes: &HashSet<(usize, usize)>) -> u32 {
    tiles
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .fold(
                    (0, BoundaryStatus::Outside),
                    |(area, status), (col_idx, cell)| {
                        let location = (row_idx, col_idx);
                        if loop_nodes.contains(&location) {
                            let new_status = match cell {
                                '|' => match status {
                                    BoundaryStatus::Inside => Some(BoundaryStatus::Outside),
                                    BoundaryStatus::Outside => Some(BoundaryStatus::Inside),
                                    _ => None,
                                }
                                .unwrap(),
                                'F' => match status {
                                    BoundaryStatus::Inside => Some(BoundaryStatus::ExitFromBelow),
                                    BoundaryStatus::Outside => Some(BoundaryStatus::EnterFromBelow),
                                    _ => None,
                                }
                                .unwrap(),
                                'L' => match status {
                                    BoundaryStatus::Inside => Some(BoundaryStatus::ExitFromAbove),
                                    BoundaryStatus::Outside => Some(BoundaryStatus::EnterFromAbove),
                                    _ => None,
                                }
                                .unwrap(),
                                'J' => match status {
                                    BoundaryStatus::EnterFromAbove => Some(BoundaryStatus::Outside),
                                    BoundaryStatus::EnterFromBelow => Some(BoundaryStatus::Inside),
                                    BoundaryStatus::ExitFromAbove => Some(BoundaryStatus::Inside),
                                    BoundaryStatus::ExitFromBelow => Some(BoundaryStatus::Outside),
                                    _ => None,
                                }
                                .unwrap(),
                                '7' => match status {
                                    BoundaryStatus::EnterFromAbove => Some(BoundaryStatus::Inside),
                                    BoundaryStatus::EnterFromBelow => Some(BoundaryStatus::Outside),
                                    BoundaryStatus::ExitFromAbove => Some(BoundaryStatus::Outside),
                                    BoundaryStatus::ExitFromBelow => Some(BoundaryStatus::Inside),
                                    _ => None,
                                }
                                .unwrap(),
                                _ => status,
                            };
                            (area, new_status)
                        } else {
                            match status {
                                BoundaryStatus::Inside => (area + 1, status),
                                _ => (area, status),
                            }
                        }
                    },
                )
                .0
        })
        .sum()
}

fn identify_loop(tiles: &Vec<Vec<char>>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let (height, width) = (tiles.len(), tiles.get(0).unwrap().len());
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert(start);

    let mut current_nodes: Vec<(u32, (usize, usize))>;
    let mut next_nodes = vec![(0, start)];
    while !next_nodes.is_empty() {
        current_nodes = next_nodes;
        next_nodes = current_nodes
            .iter()
            .flat_map(|&(steps, node)| {
                let (row_index, col_index) = node;
                let tile = tiles.get(row_index).unwrap().get(col_index).unwrap();
                let neighbor_candidates = expand(tile, node);
                let validated_neighbors: Vec<(u32, (usize, usize))> = neighbor_candidates
                    .iter()
                    .filter(|candidate| !seen.contains(candidate))
                    .filter(|(row_index, col_index)| {
                        (0..height).contains(row_index) && (0..width).contains(col_index)
                    })
                    .map(|&(row_index, col_index)| {
                        // Valid position
                        let tile = tiles.get(row_index).unwrap().get(col_index).unwrap();
                        (tile, (row_index, col_index))
                    })
                    .filter(|&(tile, candidate)| expand(tile, candidate).contains(&node))
                    .map(|(_tile, candidate)| (steps + 1, candidate))
                    .collect();
                validated_neighbors
            })
            .collect();
        next_nodes.iter().for_each(|&(_, node)| {
            seen.insert(node);
        });
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
