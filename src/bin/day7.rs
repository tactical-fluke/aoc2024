use std::iter::zip;
use std::ptr::eq;
use std::slice::Iter;
use itertools::Itertools;
use aoc2024::day::{run_day, Day};
use crate::Op::Cat;

fn main() {
    let results = run_day::<Day7>();
    println!("part1:{:?}, part2:{:?}", results.0, results.1);
}

struct Equation {
    target: u64,
    terms: Vec<u64>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
    Cat,
}

impl Op {
    pub fn iterator() -> Iter<'static, Op> {
        [Op::Add, Op::Mul].iter()
    }

    pub fn iterator_with_cat() -> Iter<'static, Op> {
        [Op::Add, Op::Mul, Op::Cat].iter()
    }
}

struct Day7 {}

impl Day for Day7 {
    const INPUT_FILE: &'static str = "day7.txt";
    type OutputType = u64;
    type ParsedType = Vec<Equation>;

    fn parse_input(input: &str) -> Self::ParsedType {
        let mut ret = Vec::new();
        for line in input.lines() {
            let parts = line.split_once(":").unwrap();
            let target = parts.0.parse().unwrap();
            let terms = parts.1.trim().split(' ').map(|x| x.parse().unwrap()).collect();
            ret.push(Equation { target, terms });
        }
        ret
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        input.iter().filter_map(|equation| {
            let target = equation.target;
            let multi_prod: Vec<Vec<Op>> = (0..equation.terms.len() - 1).map(|_| Op::iterator().cloned())
                .multi_cartesian_product().collect();
            for perm in multi_prod.iter() {
                let mut actual = equation.terms[0];
                for i in 1..equation.terms.len() {
                    match perm[i  -1] {
                        Op::Add => actual += equation.terms[i],
                        Op::Mul => actual *= equation.terms[i],
                        Op::Cat => panic!()
                    }
                }
                if actual == target {
                    return Some(target);
                }
            }
            None
        }).sum()
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
        input.iter().filter_map(|equation| {
            let target = equation.target;
            let multi_prod: Vec<Vec<Op>> = (0..equation.terms.len() - 1).map(|_| Op::iterator_with_cat().cloned())
                .multi_cartesian_product().collect();
            for perm in multi_prod.iter() {
                let mut actual = equation.terms[0];
                for i in 1..equation.terms.len() {
                    match perm[i  -1] {
                        Op::Add => actual += equation.terms[i],
                        Op::Mul => actual *= equation.terms[i],
                        Cat => actual = concat(actual, equation.terms[i]),
                    }
                }
                if actual == target {
                    return Some(target);
                }
            }
            None
        }).sum()
    }
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse().unwrap() //HACK I GUESS LMAO
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    #[test]
    fn test_part1() {
        const TEST_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let input = Day7::parse_input(TEST_INPUT);
        assert_eq!(Day7::part1(&input), 3749);
    }

    #[test]
    fn test_part2() {
        const TEST_INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let input = Day7::parse_input(TEST_INPUT);
        assert_eq!(Day7::part2(&input), 11387);
    }
}
