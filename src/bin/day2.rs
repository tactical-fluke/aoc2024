use std::cmp::Ordering;
use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day2>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

struct Day2 {}

impl Day for Day2 {
    const INPUT_FILE: &'static str = "day2.txt";
    type OutputType = i32;
    type ParsedType = Vec<Vec<i32>>;

    fn parse_input(input: &str) -> Self::ParsedType {
        input
            .lines()
            .map(|l| l.split(' ').map(|item| item.parse().unwrap()).collect())
            .collect()
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        let mut result: Vec<bool> = Vec::with_capacity(input.len());
        for report in input {
            let ord = report[1].cmp(&report[0]);
            let mut this_result = true;
            for i in 1..report.len() {
                let diff = report[i] - report[i - 1];
                if report[i].cmp(&report[i - 1]) != ord || diff.abs() > 3 {
                    this_result = false;
                    break;
                }
            }
            result.push(this_result);
        }
        result.iter().filter(|x| **x).count() as Self::OutputType
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
        let mut result: Vec<bool> = Vec::with_capacity(input.len());
        for report in input {
            let ord = report[1].cmp(&report[0]);
            let mut this_result = true;
            let mut level_dampened = false;
            for i in 1..report.len() {
                let diff = report[i] - report[i - 1];
                if report[i].cmp(&report[i - 1]) != ord || diff.abs() > 3 {
                    if level_dampened {
                        this_result = false;
                        break;
                    }
                    let diff = report[i] - report[i - 2];
                    if report[i].cmp(&report[i - 2]) != ord || diff.abs() > 3 {
                        this_result = false;
                        break;
                    }
                    level_dampened = true;
                    continue;
                }
            }
            result.push(this_result);
        }
        result.iter().filter(|x| **x).count() as Self::OutputType
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let parsed = Day2::parse_input(input);
        assert_eq!(Day2::part1(&parsed), 2);
    }

    #[test]
    fn test_part2() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let parsed = Day2::parse_input(input);
        assert_eq!(Day2::part2(&parsed), 4);
    }
}
