use std::iter::zip;
use std::ptr::eq;
use std::slice::Iter;
use itertools::Itertools;
use aoc2024::day::{run_day, Day};
use crate::Op::Cat;

fn main() {
    let results = run_day::<Day8>();
    //println!("part1:{:?}, part2:{:?}", results.0, results.1);
}

struct Day8 {}

impl Day for Day8 {
    const INPUT_FILE: &'static str = "day7.txt";
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
mod day8_tests {
    use super::*;

    #[test]
    fn test_part1() {
    }

    #[test]
    fn test_part2() {
    }
}
