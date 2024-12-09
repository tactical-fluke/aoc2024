use itertools::Itertools;
use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day9>();
    println!("part1:{:?}, part2:{:?}", results.0, results.1);
}

struct Day9 {}

impl Day for Day9 {
    const INPUT_FILE: &'static str = "day9.txt";
    type OutputType = u64;
    type ParsedType = Vec<Option<u64>>;

    fn parse_input(input: &str) -> Self::ParsedType {
        let mut ret = Vec::new();
        let mut iter = input.chars().peekable();
        let mut id = 0;
        while iter.peek().is_some() {
            let size = iter.next().unwrap().to_string().parse::<u32>().unwrap();
            let space  = iter.next().unwrap_or('0').to_string().parse::<u32>().unwrap();
            for _ in 0..size {
                ret.push(Some(id));
            }
            for _ in 0..space {
                ret.push(None);
            }
            id += 1;
        }
        ret
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        let mut compacted = input.clone();
        let mut space_index = 0usize;
        let mut id_index = compacted.len() - 1;

        loop {
            if space_index == id_index {
                break;
            }
            if let Some(None) = compacted.get(id_index) {
                id_index -= 1;
                continue
            }
            if let Some(Some(_)) = compacted.get(space_index) {
                space_index += 1;
                continue;
            }
            compacted.swap(id_index, space_index);
            space_index += 1;
            id_index -= 1;
        }

        let filtered = compacted.iter()
            .enumerate()
            .filter(|(_, s)| s.is_some())
            .collect::<Vec<_>>();
            filtered.iter().map(|(idx, item)| item.unwrap() * (*idx as u64))
            .sum()
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
        0
    }
}

#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn test_part1() {
        const TEST_INPUT: &str = "2333133121414131402";

        let input = Day9::parse_input(TEST_INPUT);
        assert_eq!(Day9::part1(&input), 1928);
    }

    #[test]
    fn test_part2() {
    }
}
