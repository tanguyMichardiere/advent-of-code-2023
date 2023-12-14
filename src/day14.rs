use cached::proc_macro::cached;

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn tilt_horizontal(platform: &mut Vec<Vec<char>>, west: bool) {
    for row in platform {
        for sub in row.split_mut(|char| char == &'#') {
            sub.sort();
            if west {
                sub.reverse();
            }
        }
    }
}

/// Apply a diagonal reflection in place, assuming the platform is a square
///
/// `\....` \
/// `.\...` \
/// `..\..` \
/// `...\.` \
/// `....\` \
///
/// Applying this twice is a no-op
fn reflect(platform: &mut Vec<Vec<char>>) {
    for y in 0..platform.len() {
        for x in 0..y {
            (platform[y][x], platform[x][y]) = (platform[x][y], platform[y][x]);
        }
    }
}

fn tilt_vertical(platform: &mut Vec<Vec<char>>, north: bool) {
    reflect(platform);
    tilt_horizontal(platform, north);
    reflect(platform)
}

fn north_load(platform: Vec<Vec<char>>) -> usize {
    let platform_len = platform.len();
    platform
        .into_iter()
        .enumerate()
        .map(|(i, row)| (platform_len - i) * row.into_iter().filter(|char| char == &'O').count())
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let mut platform = parse(input);
    tilt_vertical(&mut platform, true);
    north_load(platform)
}

#[cached(key = "u64", convert = "{ crate::cache::hash((&platform, count)) }")]
fn cycled(mut platform: Vec<Vec<char>>, count: usize) -> Vec<Vec<char>> {
    if count == 1 {
        tilt_vertical(&mut platform, true);
        tilt_horizontal(&mut platform, true);
        tilt_vertical(&mut platform, false);
        tilt_horizontal(&mut platform, false);
        platform
    } else {
        let count = count / 10;
        for _ in 0..10 {
            platform = cycled(platform, count);
        }
        platform
    }
}

pub fn part_two(input: &str) -> usize {
    north_load(cycled(parse(input), 1000000000))
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/14/1").unwrap();
        assert_eq!(part_one(&input), 136);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/14/1").unwrap();
        assert_eq!(part_two(&input), 64);
    }
}
