advent_of_code::solution!(7);

use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?<hand>\w{5}) (?<bet>\d+)").unwrap();
    let (hands, bets): (Vec<&str>, Vec<u32>) = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap.name("hand").unwrap().as_str(),
                cap.name("bet").unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .unzip();
    let hand_scores = hands.iter().map(hand_score).collect_vec();
    let indices_ranked: Vec<usize> = hand_scores
        .iter()
        .enumerate()
        .sorted_by_key(|(_idx, score)| *score)
        .map(|(old_idx, _score)| old_idx)
        .collect();
    let results: Vec<u32> = indices_ranked
        .iter()
        .enumerate()
        .map(|(rank, old_idx)| (rank as u32 + 1) * bets.get(*old_idx).unwrap())
        .collect();
    let result = results.iter().sum();
    Some(result)
}

fn hand_score(hand: &&str) -> (u32, u32) {
    let gs = type_score(hand);
    let tbs = tie_breaker_score(hand);
    (gs, tbs)
}

fn type_score(hand: &&str) -> u32 {
    let mut card_counts: HashMap<char, u32> = HashMap::new();
    hand.chars().for_each(|c| {
        card_counts.insert(c, *card_counts.get(&c).unwrap_or(&0) + 1);
    });
    let repeat_counts: Vec<&u32> = card_counts.values().sorted().rev().collect();
    match repeat_counts[..] {
        [2, 1, 1, 1] => 1,
        [2, 2, 1] => 2,
        [3, 1, 1] => 3,
        [3, 2] => 4,
        [4, 1] => 5,
        [5] => 6,
        _ => 0,
    }
}

fn tie_breaker_score(hand: &&str) -> u32 {
    const POWERS: [u32; 5] = [
        50625, // 15^4
        3375,  // 15^3
        225,   // 15^2
        15,    // 15^1
        1,     // 15^0
    ];

    hand.char_indices()
        .map(|(i, c)| POWERS.get(i).unwrap() * card_score(c))
        .sum()
}

fn card_score(card: char) -> u32 {
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        c => c.to_digit(10).unwrap_or_default(),
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?<hand>\w{5}) (?<bet>\d+)").unwrap();
    let (hands, bets): (Vec<&str>, Vec<u32>) = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap.name("hand").unwrap().as_str(),
                cap.name("bet").unwrap().as_str().parse::<u32>().unwrap(),
            )
        })
        .unzip();
    let hand_scores = hands.iter().map(hand_score_with_joker).collect_vec();
    let indices_ranked: Vec<usize> = hand_scores
        .iter()
        .enumerate()
        .sorted_by_key(|(_idx, score)| *score)
        .map(|(old_idx, _score)| old_idx)
        .collect();
    let results: Vec<u32> = indices_ranked
        .iter()
        .enumerate()
        .map(|(rank, old_idx)| (rank as u32 + 1) * bets.get(*old_idx).unwrap())
        .collect();
    let result = results.iter().sum();
    Some(result)
}

fn hand_score_with_joker(hand: &&str) -> (u32, u32) {
    let gs = type_score_with_joker(hand);
    let hand_with_jokers_replaced = hand.replace('J', "1");
    let tbs = tie_breaker_score(&&hand_with_jokers_replaced[..]);
    (gs, tbs)
}

fn type_score_with_joker(hand: &&str) -> u32 {
    let mut card_counts: HashMap<char, u32> = HashMap::new();
    hand.chars().for_each(|c| {
        card_counts.insert(c, *card_counts.get(&c).unwrap_or(&0) + 1);
    });
    if card_counts.contains_key(&'J') && card_counts.len() > 1 {
        // Replace the top non-joker card
        let top_char = card_counts
            .iter()
            .filter(|(k, _v)| **k != 'J')
            .max_by_key(|(_k, v)| **v)
            .unwrap()
            .0;
        card_counts.insert(
            *top_char,
            *card_counts.get(top_char).unwrap() + *card_counts.get(&'J').unwrap(),
        );
        card_counts.remove(&'J');
    }
    let repeat_counts: Vec<&u32> = card_counts.values().sorted().rev().collect();
    match repeat_counts[..] {
        [2, 1, 1, 1] => 1,
        [2, 2, 1] => 2,
        [3, 1, 1] => 3,
        [3, 2] => 4,
        [4, 1] => 5,
        [5] => 6,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
