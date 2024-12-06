use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day6>();
    //println!("part1:{}, part2:{}", results.0, results.1);
}

struct Day6 {}

impl Day for Day6 {
    const INPUT_FILE: &'static str = "day6.txt";
    type OutputType = ();
    type ParsedType = ();

    fn parse_input(input: &str) -> Self::ParsedType {
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
    }
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn test_part1() {
    }

    #[test]
    fn test_part2() {
    }
}
