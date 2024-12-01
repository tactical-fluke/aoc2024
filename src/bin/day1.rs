use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day1>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

struct Day1 {}

impl Day for Day1 {
    const INPUT_FILE: &'static str = "day1.txt";
    type OutputType = u32;
    type ParsedType = (Vec<u32>, Vec<u32>);

    fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
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

    fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
        let (list_1, list_2) = input;
        list_1.iter().zip(list_2.iter()).map(|(a, b)| u32::abs_diff(*a, *b)).sum()
    }

    fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
        let (list_1, list_2) = input;
        list_1.iter()
            .map(|item| item * list_2.iter().filter(|item2| &item == item2).count() as u32)
            .sum()
    }
}
