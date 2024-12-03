use regex::Regex;
use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day3>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

struct Day3 {}

impl Day for Day3 {
    const INPUT_FILE: &'static str = "day3.txt";
    type OutputType = u32;
    type ParsedType = Vec<(u32, u32)>;

    fn parse_input(input: &str) -> Self::ParsedType {
        let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
        regex.captures_iter(input)
            .map(|caps| {
                let (_, [val1, val2]) = caps.extract();
                (val1.parse().unwrap(), val2.parse().unwrap())
            }).collect()
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        input.iter().map(|(val1, val2)| val1 * val2).sum()
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
       0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part1() {
        let input = Day3::parse_input(TEST_STRING);
        assert_eq!(Day3::part1(&input), 161);
    }

    #[test]
    fn test_part2() {
    }
}
