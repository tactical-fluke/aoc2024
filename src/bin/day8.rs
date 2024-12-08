use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use nalgebra::Vector2;
use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day8>();
    println!("part1:{:?}, part2:{:?}", results.0, results.1);
}

struct Map {
    extents: Vector2<i32>,
    antennas: HashMap<char, HashSet<Vector2<i32>>>,
}

struct Day8 {}

impl Day for Day8 {
    const INPUT_FILE: &'static str = "day8.txt";
    type OutputType = usize;
    type ParsedType = Map;

    fn parse_input(input: &str) -> Self::ParsedType {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut antennas = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                max_x = max_x.max(x);
                if c == '.' {
                    continue;
                }
                antennas.entry(c).or_insert_with(HashSet::new).insert(Vector2::new(x as i32, y as i32));
            }
            max_y = max_y.max(y);
        }
        Map {
            extents: Vector2::new(max_x as i32, max_y as i32),
            antennas
        }
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        let mut unique_antinode_positions: HashSet<Vector2<i32>> = HashSet::new();
        for node_positions in input.antennas.values() {
            for position_pair in node_positions.iter().permutations(2) {
                let diff = position_pair[0] - position_pair[1];
                let anti_node = position_pair[0] + diff;
                if check_pos_inside_extents(&input.extents, &anti_node) {
                    unique_antinode_positions.insert(anti_node);
                }
                let antinode = position_pair[1] - diff;
                if check_pos_inside_extents(&input.extents, &antinode) {
                    unique_antinode_positions.insert(antinode);
                }
            }
        }
        unique_antinode_positions.len()
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
        let mut unique_antinode_positions: HashSet<Vector2<i32>> = HashSet::new();
        for node_positions in input.antennas.values() {
            for position_pair in node_positions.iter().permutations(2) {
                let diff = position_pair[0] - position_pair[1];
                let mut index = 0;
                loop {
                    let anti_node = position_pair[0] + diff * index;
                    if check_pos_inside_extents(&input.extents, &anti_node) {
                        unique_antinode_positions.insert(anti_node);
                    } else {
                        break;
                    }
                    index += 1;
                }
                index = 0;
                loop {
                    let anti_node = position_pair[1] - diff * index;
                    if check_pos_inside_extents(&input.extents, &anti_node) {
                        unique_antinode_positions.insert(anti_node);
                    } else {
                        break;
                    }
                    index += 1;
                }
            }
        }
        unique_antinode_positions.len()
    }
}

fn check_pos_inside_extents(extents: &Vector2<i32>, position: &Vector2<i32>) -> bool {
    position.x <= extents.x && position.y <= extents.y && position.x >= 0 && position.y >= 0
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let input = Day8::parse_input(INPUT);
        let result = Day8::part1(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let input = Day8::parse_input(INPUT);
        let result = Day8::part2(&input);
        assert_eq!(result, 34);
    }
}
