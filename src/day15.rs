use std::collections::HashMap;

use itertools::Itertools;

use crate::regex;

fn hash(string: &str) -> usize {
    let mut result = 0;
    for char in string.as_bytes() {
        result += *char as usize;
        result *= 17;
        result %= 256;
    }
    result
}

pub fn part_one(input: &str) -> usize {
    regex!(r"(?P<step>[[:ascii:]--[,]]+)[,\n]")
        .captures_iter(input)
        .map(|caps| hash(&caps["step"]))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut boxes = HashMap::<usize, Vec<(String, usize)>>::new();
    for caps in regex!(r"(?P<label>[a-z]+)(?P<operation>-|(?:=\d))[,\n]").captures_iter(input) {
        let lens_label = caps["label"].to_owned();
        let operation = &caps["operation"];
        let operation = if operation.starts_with('=') {
            Some(operation.chars().nth(1).unwrap().to_digit(10).unwrap() as usize)
        } else {
            None
        };
        let box_index = hash(&lens_label);
        match boxes.get_mut(&box_index) {
            Some(box_content) => {
                match box_content
                    .iter()
                    .find_position(|(old_lens_label, _)| old_lens_label == &lens_label)
                {
                    Some((position, _)) => {
                        if let Some(focal_length) = operation {
                            box_content[position] = (lens_label, focal_length);
                        } else {
                            box_content.remove(position);
                        }
                    }
                    None => {
                        if let Some(focal_length) = operation {
                            box_content.push((lens_label, focal_length));
                        }
                    }
                }
            }
            None => {
                if let Some(focal_length) = operation {
                    boxes.insert(box_index, vec![(lens_label, focal_length)]);
                }
            }
        }
    }
    boxes
        .into_iter()
        .flat_map(|(box_number, lenses)| {
            lenses
                .into_iter()
                .enumerate()
                .map(move |(lens_slot, (_, focal_length))| {
                    (box_number + 1) * (lens_slot + 1) * focal_length
                })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/15/1").unwrap();
        assert_eq!(part_one(&input), 1320);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/15/1").unwrap();
        assert_eq!(part_two(&input), 145);
    }
}
