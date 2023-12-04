use std::collections::{HashMap, HashSet};

use crate::regex;

pub fn part_one(input: &str) -> u32 {
    regex!(r"(?m)^Card +(?P<card>\d+): +(?P<winning>[\d ]+) +\| +(?P<numbers>[\d ]+)$")
        .captures_iter(input)
        .map(|caps| {
            let winning = regex!(r"(\d+)")
                .captures_iter(&caps["winning"])
                .map(|caps| caps[1].parse::<u32>().unwrap())
                .collect::<HashSet<_>>();
            let matches = regex!(r"(\d+)")
                .captures_iter(&caps["numbers"])
                .map(|caps| caps[1].parse::<u32>().unwrap())
                .filter(|number| winning.contains(number))
                .count() as u32;
            if matches > 0 {
                2u32.pow(matches - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut copies = HashMap::<u32, u32>::new();
    for caps in regex!(r"(?m)^Card +(?P<card>\d+): +(?P<winning>[\d ]+) +\| +(?P<numbers>[\d ]+)$")
        .captures_iter(input)
    {
        let card = caps["card"].parse::<u32>().unwrap();
        let card_count = *copies.entry(card).or_insert(1);
        let winning = regex!(r"(\d+)")
            .captures_iter(&caps["winning"])
            .map(|caps| caps[1].parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let matches = regex!(r"(\d+)")
            .captures_iter(&caps["numbers"])
            .map(|caps| caps[1].parse::<u32>().unwrap())
            .filter(|number| winning.contains(number))
            .count() as u32;
        for i in 0..matches {
            *copies.entry(card + 1 + i).or_insert(1) += card_count;
        }
    }
    copies.values().sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/04/1").unwrap();
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/04/1").unwrap();
        assert_eq!(part_two(&input), 30);
    }
}
