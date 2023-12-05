use std::cmp::Ordering::{Equal, Greater, Less};

use crate::regex;

type Range = std::ops::Range<u64>;

fn parse_maps(input: &str) -> Vec<Vec<(Range, i64)>> {
    regex!(r"\w+-to-\w+ map:\n(?P<map>(?:\d+ \d+ \d+\n)+)")
        .captures_iter(input)
        .map(|caps| {
            regex!(r"(?P<destination_range_start>\d+) (?P<source_range_start>\d+) (?P<range_length>\d+)")
                .captures_iter(&caps["map"])
                .map(|caps| {
                        let range_start = caps["source_range_start"].parse::<u64>().unwrap();
                        (range_start..(range_start + caps["range_length"].parse::<u64>().unwrap()), caps["destination_range_start"].parse::<i64>().unwrap() - range_start as i64)
                })
                .collect()
        })
        .collect()
}

fn transformed(number: u64, transform: i64) -> u64 {
    ((number as i64) + transform) as u64
}

pub fn part_one(input: &str) -> u64 {
    let caps = regex!(
        r"seeds: (?P<seeds>[\d ]+)\n\n(?P<maps>(?:\w+-to-\w+ map:\n(?:\d+ \d+ \d+\n)+\n?)+)"
    )
    .captures(input)
    .unwrap();
    let maps = parse_maps(&caps["maps"]);
    regex!(r"(?<seed>\d+)")
        .captures_iter(&caps["seeds"])
        .map(|caps| caps["seed"].parse::<u64>().unwrap())
        .map(|mut number| {
            for map in &maps {
                for (range, transform) in map {
                    if range.contains(&number) {
                        number = transformed(number, *transform);
                        break;
                    }
                }
            }
            number
        })
        .min()
        .unwrap()
}

struct SparseRange(Vec<Range>);

impl SparseRange {
    /// Extract a range from the sparse range, truncating it in place and returning the extract
    ///
    /// This can sometimes make the sparse range sparser, if the extract range is contained in one of the sparse range's ranges
    ///
    /// In that case the containing range is truncated towards its start, and what remains towards its end is added at the end of the sparse range
    fn extract(&mut self, extract_range: &Range) -> Vec<Range> {
        let mut new_range = None;
        let mut extract = Vec::new();
        for range in self.0.iter_mut() {
            if range.end > extract_range.start && range.start < extract_range.end {
                match (
                    range.start.cmp(&extract_range.start),
                    range.end.cmp(&extract_range.end),
                ) {
                    (Less, Less | Equal) => {
                        // source  -----
                        // extract    -----
                        extract.push(extract_range.start..range.end);
                        range.end = extract_range.start;
                    }
                    (Less, Greater) => {
                        // source  -------
                        // extract  -----
                        extract.push(extract_range.clone());
                        new_range = Some(extract_range.end..range.end);
                        range.end = extract_range.start;
                    }
                    (Greater | Equal, Less | Equal) => {
                        // source   -----
                        // extract -------
                        extract.push(range.clone());
                        range.end = range.start;
                    }
                    (Equal, Greater) => {
                        // source  -------
                        // extract -----
                        extract.push(extract_range.clone());
                        range.start = extract_range.end;
                    }
                    (Greater, Greater) => {
                        // source     -----
                        // extract -----
                        extract.push(range.start..extract_range.end);
                        range.start = extract_range.end;
                    }
                }
            }
        }
        self.0.extend(new_range);
        self.0.retain(|range| range.end > range.start);
        extract
    }

    fn apply_transform(&mut self, map: &[(Range, i64)]) {
        let mut new_ranges = Vec::new();
        for (transform_range, transform) in map {
            new_ranges.extend(self.extract(transform_range).into_iter().map(|range| {
                transformed(range.start, *transform)..transformed(range.end, *transform)
            }));
        }
        self.0.extend(new_ranges);
    }
}

pub fn part_two(input: &str) -> u64 {
    let caps = regex!(
        r"seeds: (?P<seeds>[\d ]+)\n\n(?P<maps>(?:\w+-to-\w+ map:\n(?:\d+ \d+ \d+\n)+\n?)+)"
    )
    .captures(input)
    .unwrap();
    let maps = parse_maps(&caps["maps"]);
    let mut ranges = SparseRange(
        regex!(r"(?P<start>\d+) (?P<length>\d+)")
            .captures_iter(&caps["seeds"])
            .map(|caps| {
                let start = caps["start"].parse::<u64>().unwrap();
                let length = caps["length"].parse::<u64>().unwrap();
                start..(start + length)
            })
            .collect(),
    );
    for map in &maps {
        ranges.apply_transform(map);
    }
    ranges.0.iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/05/1").unwrap();
        assert_eq!(part_one(&input), 35);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/05/1").unwrap();
        assert_eq!(part_two(&input), 46);
    }
}
