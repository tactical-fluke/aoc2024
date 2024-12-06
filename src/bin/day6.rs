use std::collections::{HashMap, HashSet};
use aoc2024::day::{run_day, Day};
use crate::MapItem::Guard;

fn main() {
    let results = run_day::<Day6>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("Unknown direction: {}", c),
        }
    }
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Self::East,
            Direction::South => Self::West,
            Direction::East => Self::South,
            Direction::West => Self::North
        }
    }
}

#[derive(Debug, Clone)]
enum MapItem {
    Empty,
    Blocked,
    Guard(Direction)
}

struct Map {
    items: Vec<Vec<MapItem>>,
    guard_position: (usize, usize),
}

impl From<char> for MapItem {
    fn from(c: char) -> Self {
        match c {
            '#' => MapItem::Blocked,
            '.' => MapItem::Empty,
            c => Guard(Direction::from(c))
        }
    }
}

struct Day6 {}

impl Day for Day6 {
    const INPUT_FILE: &'static str = "day6.txt";
    type OutputType = usize;
    type ParsedType = Map;

    fn parse_input(input: &str) -> Self::ParsedType {
        let mut ret = Vec::new();
        let mut position = (0, 0);
        for (y_index, line) in input.lines().enumerate() {
            let mut map_line: Vec<MapItem> = Vec::new();
            for (x_index, c) in line.chars().enumerate() {
                let map_item = MapItem::from(c);
                if let Guard(_) = map_item {
                    position = (x_index, y_index);
                }
                map_line.push(map_item);
            }
            ret.push(map_line);
        }
        Map { items: ret, guard_position: position }
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        let mut unique_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut guard_direction = if let Guard(dir) = input.items[input.guard_position.1][input.guard_position.0].clone() {
            dir
        } else { panic!("Guard not found"); };
        let mut guard_position = input.guard_position;
        unique_positions.insert(guard_position);
        loop {
            match guard_direction {
                Direction::North => {
                    let next_position = input.items.get(guard_position.1 - 1)
                        .map(|row| row.get(guard_position.0).map(|c| c)).flatten();
                    match next_position {
                        Some(MapItem::Blocked) => guard_direction = guard_direction.rotate(),
                        Some(_) => guard_position.1 -= 1,
                        None => break
                    }
                }
                Direction::South => {
                    let next_position = input.items.get(guard_position.1 + 1)
                        .map(|row| row.get(guard_position.0).map(|c| c)).flatten();
                    match next_position{
                        Some(MapItem::Blocked) => guard_direction = guard_direction.rotate(),
                        Some(_) => guard_position.1 += 1,
                        None => break
                    }
                }
                Direction::East => {
                    let next_position = input.items.get(guard_position.1)
                        .map(|row| row.get(guard_position.0 + 1).map(|c| c)).flatten();
                    match next_position {
                        Some(MapItem::Blocked) => guard_direction = guard_direction.rotate(),
                        Some(_) => guard_position.0 += 1,
                        None => break
                    }
                }
                Direction::West => {
                    let next_position = input.items.get(guard_position.1)
                        .map(|row| row.get(guard_position.0 - 1).map(|c| c)).flatten();
                    match next_position {
                        Some(MapItem::Blocked) => guard_direction = guard_direction.rotate(),
                        Some(_) => guard_position.0 -= 1,
                        None => break
                    }
                }
            }
            unique_positions.insert(guard_position);
        }
        unique_positions.len()
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
        0
    }
}

#[cfg(test)]
mod day6_tests {
    use super::*;

    #[test]
    fn test_part1() {
        const TEST_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        
        let input = Day6::parse_input(TEST_INPUT);
        assert_eq!(Day6::part1(&input), 41);
    }

    #[test]
    fn test_part2() {
    }
}
