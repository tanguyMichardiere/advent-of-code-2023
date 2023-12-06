use std::ops::Range;

use crate::regex;

/// Solves the quadratic equation `x * (time - x) > distance` and returns the whole solutions
fn solve(time: u64, distance: u64) -> Range<u64> {
    let delta = ((time.pow(2) - 4 * distance) as f64).sqrt();
    let min = (time as f64 - delta) / 2.;
    let max = (time as f64 + delta) / 2.;
    ((min + 1.) as u64)..(max.ceil() as u64)
}

pub fn part_one(input: &str) -> usize {
    let caps = regex!(r"Time:(?P<times>(?: +\d+)+)\nDistance:(?P<distances>(?: +\d+)+)")
        .captures(input)
        .unwrap();
    regex!(r"(?P<time>\d+)")
        .captures_iter(&caps["times"])
        .zip(regex!(r"(?P<distance>\d+)").captures_iter(&caps["distances"]))
        .map(|(time, distance)| {
            solve(
                time["time"].parse::<u64>().unwrap(),
                distance["distance"].parse::<u64>().unwrap(),
            )
            .count()
        })
        .product()
}

pub fn part_two(input: &str) -> usize {
    let well_kerned_input = input.replace(" ", "");
    let caps = regex!(r"Time:(?P<time>\d+)\nDistance:(?P<distance>\d+)")
        .captures(&well_kerned_input)
        .unwrap();
    solve(
        caps["time"].parse::<u64>().unwrap(),
        caps["distance"].parse::<u64>().unwrap(),
    )
    .count()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/06/1").unwrap();
        assert_eq!(part_one(&input), 288);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/06/1").unwrap();
        assert_eq!(part_two(&input), 71503);
    }
}
