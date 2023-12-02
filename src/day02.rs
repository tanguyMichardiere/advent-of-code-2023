use crate::regex;

pub fn part_one(input: &str) -> u32 {
    regex!(r"(?m)Game (?P<game>\d+): (?P<cubes>.+)")
        .captures_iter(input)
        .filter_map(|caps| {
            let game = caps["game"].parse::<u32>().unwrap();
            if regex!(r"(?P<set>\d[^;]+)")
                .captures_iter(&caps["cubes"])
                .all(|caps| {
                    regex!(r"(?P<count>\d+) (?P<color>red|green|blue)")
                        .captures_iter(&caps["set"])
                        .all(|caps| {
                            caps["count"].parse::<u32>().unwrap()
                                <= match &caps["color"] {
                                    "red" => 12,
                                    "green" => 13,
                                    "blue" => 14,
                                    _ => unreachable!(),
                                }
                        })
                })
            {
                Some(game)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    regex!(r"(?m)Game (?P<game>\d+): (?P<cubes>.+)")
        .captures_iter(input)
        .map(|caps| {
            let res = regex!(r"(?P<set>\d[^;]+)")
                .captures_iter(&caps["cubes"])
                .map(|caps| {
                    regex!(r"(?P<count>\d+) (?P<color>red|green|blue)")
                        .captures_iter(&caps["set"])
                        .fold((0, 0, 0), |res, caps| {
                            let count = caps["count"].parse::<u32>().unwrap();
                            match &caps["color"] {
                                "red" => (res.0.max(count), res.1, res.2),
                                "green" => (res.0, res.1.max(count), res.2),
                                "blue" => (res.0, res.1, res.2.max(count)),
                                _ => unreachable!(),
                            }
                        })
                })
                .fold((0, 0, 0), |res, colors| {
                    (
                        res.0.max(colors.0),
                        res.1.max(colors.1),
                        res.2.max(colors.2),
                    )
                });
            res.0 * res.1 * res.2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/02/1").unwrap();
        assert_eq!(part_one(&input), 8);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/02/1").unwrap();
        assert_eq!(part_two(&input), 2286);
    }
}
