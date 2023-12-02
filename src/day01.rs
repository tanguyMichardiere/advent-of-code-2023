use crate::regex;

pub fn part_one(input: &str) -> u32 {
    regex!(r"(?m)(?P<first>\d)(?:.*(?P<last>\d))?")
        .captures_iter(input)
        .map(|caps| {
            let first = caps["first"].parse::<u32>().unwrap();
            10 * first
                + caps
                    .name("last")
                    .map(|last| last.as_str().parse().unwrap())
                    .unwrap_or(first)
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    fn parse_digit(input: &str) -> u32 {
        match input {
            "0" => 0,
            "1" | "one" => 1,
            "2" | "two" => 2,
            "3" | "three" => 3,
            "4" | "four" => 4,
            "5" | "five" => 5,
            "6" | "six" => 6,
            "7" | "seven" => 7,
            "8" | "eight" => 8,
            "9" | "nine" => 9,
            _ => unreachable!(),
        }
    }

    regex!(
        r"(?m)(?P<first>\d|one|two|three|four|five|six|seven|eight|nine)(?:.*(?P<last>\d|one|two|three|four|five|six|seven|eight|nine))?"
    )
    .captures_iter(input)
    .map(|caps| {
        let first = parse_digit(&caps["first"]);
        10 * first
            + caps
                .name("last")
                .map(|last| parse_digit(last.as_str()))
                .unwrap_or(first)
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/01/1").unwrap();
        assert_eq!(part_one(&input), 142);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/01/2").unwrap();
        assert_eq!(part_two(&input), 281);
    }
}
