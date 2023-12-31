advent_of_code::solution!(6);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let time_regex = Regex::new(r"Time:\s*(?:\d+\s+)+").unwrap();
    let distance_regex = Regex::new(r"Distance:\s*(?:\d+\s+)+").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let time_line = time_regex.find(input).unwrap().as_str();
    let distance_line = distance_regex.find(input).unwrap().as_str();
    let all_times: Vec<u32> = number_regex
        .find_iter(time_line)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();
    let all_distances: Vec<u32> = number_regex
        .find_iter(distance_line)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();

    /*
    Parameters:
    T: race time
    D: distance to beat

    Variable:
    t: Time we push the button

    Distance traveled:
    d = v_travel * t_travel
    v_travel = a * t
    t_travel = T - t
    d = (a * t) * (T - t)
    d = a * t * T - a * t * t
    d = a * t * T - a * t ^ 2

    Conditions:
    0 <= t <= T
    a = 1m/s^2
    d > D

    Substituting a = 1 and imposing d > D.
    -1 * t^2 + T * t > D
    -1 * t^2 + T * t - D > 0

    Since the quadratic equation has a negative square term,
    the only positive terms will be those between the zeroes of the equation.

    Given the quadratic formula for the zeros of an equation as below:
    a * t^2 + b * t + c = 0

    t1 = (-b + sqrt(b^2 - 4ac)) / 2a
    t2 = (-b - sqrt(b^2 - 4ac)) / 2a

    Substituting a = -1, b = T, c = -D
    t1 = (T - sqrt(T^2 - 4D)) / 2
    t2 = (T + sqrt(T^2 - 4D)) / 2

    So all times in between (t1, t2) are the solution.
    */
    let all_results: Vec<u32> = all_times
        .iter()
        .zip(all_distances.iter())
        .map(|(t, d)| solve(&(*t as u64), &(*d as u64)))
        .map(|x| x as u32)
        .collect();
    let result = all_results.iter().product::<u32>();
    Some(result)
}

fn quadratic_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let r1 = (-b + (b.powi(2) - 4f64 * a * c).sqrt()) / (2f64 * a);
    let r2 = (-b - (b.powi(2) - 4f64 * a * c).sqrt()) / (2f64 * a);
    (r1, r2)
}

fn solve(race_time: &u64, distance_to_beat: &u64) -> u64 {
    let (r1, r2) = quadratic_roots(-1f64, *race_time as f64, -(*distance_to_beat as f64));
    let lower_bound = match r1.fract() {
        x if x == 0f64 => r1.ceil() as i64 + 1,
        _ => r1.ceil() as i64,
    };
    let upper_bound = match r2.fract() {
        x if x == 0f64 => r2.floor() as i64 - 1,
        _ => r2.floor() as i64,
    };
    (upper_bound - lower_bound + 1) as u64
}

pub fn part_two(input: &str) -> Option<u32> {
    let time_regex = Regex::new(r"Time:\s*(?:\d+\s+)+").unwrap();
    let distance_regex = Regex::new(r"Distance:\s*(?:\d+\s+)+").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let time_line = time_regex.find(input).unwrap().as_str();
    let distance_line = distance_regex.find(input).unwrap().as_str();
    let all_times: Vec<&str> = number_regex
        .find_iter(time_line)
        .map(|m| m.as_str())
        .collect();
    let all_distances: Vec<&str> = number_regex
        .find_iter(distance_line)
        .map(|m| m.as_str())
        .collect();

    let actual_time = all_times.join("").parse::<u64>().unwrap();
    let actual_distance_to_beat = all_distances.join("").parse::<u64>().unwrap();
    let result = solve(&actual_time, &actual_distance_to_beat);
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
