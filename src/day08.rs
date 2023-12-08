use std::collections::HashMap;

use num::integer::lcm;

use crate::regex;

pub fn part_one(input: &str) -> usize {
    let caps =
        regex!(r"(?P<instructions>[LR]+)\n\n(?P<nodes>(?:[A-Z]{3} = \([A-Z]{3}, [A-Z]{3}\)\n)+)")
            .captures(input)
            .unwrap();
    let nodes = regex!(r"(?P<node>[A-Z]{3}) = \((?P<left>[A-Z]{3}), (?P<right>[A-Z]{3})\)")
        .captures_iter(&caps["nodes"])
        .map(|caps| {
            (
                caps["node"].to_owned(),
                (caps["left"].to_owned(), caps["right"].to_owned()),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut current_node = "AAA";
    for (step, instruction) in caps["instructions"].chars().cycle().enumerate() {
        if current_node == "ZZZ" {
            return step;
        }
        let (left, right) = &nodes[&current_node.to_owned()];
        current_node = match instruction {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        };
    }
    unreachable!()
}

pub fn part_two(input: &str) -> usize {
    let caps = regex!(
        r"(?P<instructions>[LR]+)\n\n(?P<nodes>(?:[A-Z0-9]{3} = \([A-Z0-9]{3}, [A-Z0-9]{3}\)\n)+)"
    )
    .captures(input)
    .unwrap();
    let nodes =
        regex!(r"(?P<node>[A-Z0-9]{3}) = \((?P<left>[A-Z0-9]{3}), (?P<right>[A-Z0-9]{3})\)")
            .captures_iter(&caps["nodes"])
            .map(|caps| {
                (
                    caps["node"].to_owned(),
                    (caps["left"].to_owned(), caps["right"].to_owned()),
                )
            })
            .collect::<HashMap<_, _>>();
    nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|mut current_node| {
            for (step, instruction) in caps["instructions"].chars().cycle().enumerate() {
                if current_node.ends_with('Z') {
                    return step;
                }
                let (left, right) = &nodes[&current_node.to_owned()];
                current_node = match instruction {
                    'L' => left,
                    'R' => right,
                    _ => unreachable!(),
                };
            }
            unreachable!()
        })
        .reduce(lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/08/1").unwrap();
        assert_eq!(part_one(&input), 2);

        let input = read_to_string("examples/08/2").unwrap();
        assert_eq!(part_one(&input), 6);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/08/3").unwrap();
        assert_eq!(part_two(&input), 6);
    }
}
