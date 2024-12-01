use std::fs;
use aoc2024::day::Day;

fn main() {
    println!("Hello, world!");

    println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    println!("{}", Day1::solve(&input));
    println!("{}", Day1Part2::solve(&input));
}

fn parse_day1_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let ids: Vec<(u32, u32)> = input.lines()
        .map(|line| line.split("   ").collect::<Vec<&str>>())
        .map(|parts| (parts[0], parts[1]))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let mut list_1: Vec<u32> = Vec::with_capacity(ids.len());
    let mut list_2: Vec<u32> = Vec::with_capacity(ids.len());
    for (a, b) in ids {
        list_1.push(a);
        list_2.push(b);
    }

    list_1.sort();
    list_2.sort();
    (list_1, list_2)
}

struct Day1 {}

impl Day<u32> for Day1 {
    fn solve(input: &str) -> u32 {
        let (list_1, list_2) = parse_day1_input(input);

        list_1.iter().zip(list_2.iter()).map(|(a, b)| u32::abs_diff(*a, *b)).sum()
    }
}

impl Day1 {}

struct Day1Part2 {}

impl Day<u32> for Day1Part2 {
    fn solve(input: &str) -> u32 {
        let (list1, list2) = parse_day1_input(input);

        list1.iter()
            .map(|item| item * list2.iter().filter(|item2| &item == item2).count() as u32)
            .sum()

    }
}
