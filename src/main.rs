use std::time::Duration;

mod cache;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod macros;

fn timed<F, O>(function: F, input: &str) -> (String, Duration)
where
    F: FnOnce(&str) -> O,
    O: std::fmt::Display,
{
    let start = std::time::Instant::now();
    let output = function(input);
    let elapsed = start.elapsed();
    (format!("{output}"), elapsed)
}

fn main() {
    let mut results = Vec::<((String, Duration), (String, Duration))>::with_capacity(25);
    let input = std::fs::read_to_string("inputs/01").unwrap();
    results.push((
        timed(day01::part_one, &input),
        timed(day01::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/02").unwrap();
    results.push((
        timed(day02::part_one, &input),
        timed(day02::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/03").unwrap();
    results.push((
        timed(day03::part_one, &input),
        timed(day03::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/04").unwrap();
    results.push((
        timed(day04::part_one, &input),
        timed(day04::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/05").unwrap();
    results.push((
        timed(day05::part_one, &input),
        timed(day05::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/06").unwrap();
    results.push((
        timed(day06::part_one, &input),
        timed(day06::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/07").unwrap();
    results.push((
        timed(day07::part_one, &input),
        timed(day07::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/08").unwrap();
    results.push((
        timed(day08::part_one, &input),
        timed(day08::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/09").unwrap();
    results.push((
        timed(day09::part_one, &input),
        timed(day09::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/10").unwrap();
    results.push((
        timed(day10::part_one, &input),
        timed(day10::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/11").unwrap();
    results.push((
        timed(day11::part_one, &input),
        timed(day11::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/12").unwrap();
    results.push((
        timed(day12::part_one, &input),
        timed(day12::part_two, &input),
    ));
    let input = std::fs::read_to_string("inputs/13").unwrap();
    results.push((
        timed(day13::part_one, &input),
        timed(day13::part_two, &input),
    ));
    for (day, ((part_one_answer, part_one_duration), (part_two_answer, part_two_duration))) in
        results.iter().enumerate()
    {
        println!("# Day {}", day + 1);
        println!("## Part 1");
        println!("{part_one_answer}");
        println!("computed in {part_one_duration:?}");
        println!("## Part 2");
        println!("{part_two_answer}");
        println!("computed in {part_two_duration:?}");
    }
    println!("# Total");
    println!(
        "computed in {:?}",
        results
            .iter()
            .map(|((_, part_one), (_, part_two))| *part_one + *part_two)
            .sum::<Duration>()
    );
}
