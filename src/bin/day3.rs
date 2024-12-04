use regex::Regex;
use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day3>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

enum Command {
    Do,
    Dont,
    Mul(u32, u32)
}

struct Day3 {}

impl Day for Day3 {
    const INPUT_FILE: &'static str = "day3.txt";
    type OutputType = u32;
    type ParsedType = Vec<Command>;

    fn parse_input(input: &str) -> Self::ParsedType {
        let regex = Regex::new(r"(mul|do|don't)\((\d{0,3}),*(\d{0,3})\)").unwrap();
        regex.captures_iter(input)
            .map(|caps| {
                let (_, [command_str, val1, val2]) = caps.extract();
                match command_str {
                    "don't" => Command::Dont,
                    "do" => Command::Do,
                    "mul" => Command::Mul(val1.parse().unwrap(), val2.parse().unwrap()),
                    _ => panic!("invalid command")
                }
            }).collect()
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        input.iter().map(|command|{
            match command {
                Command::Mul(val1, val2) => *val1 * *val2,
                _ => 0
            }
        }).sum()
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
       let mut enabled = true;
        let mut sum = 0;
        for command in input {
            match command {
                Command::Do => {enabled = true;},
                Command::Dont => {enabled = false;},
                Command::Mul(val1, val2) => {
                    if enabled {
                        sum += val1 * val2;
                    }
                },
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const TEST_STRING: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let input = Day3::parse_input(TEST_STRING);
        assert_eq!(Day3::part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        const TEST_STRING: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let input = Day3::parse_input(TEST_STRING);
        assert_eq!(Day3::part2(&input), 48);
    }
}
