use std::collections::HashMap;
use regex::Regex;
use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day4>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

struct SearchMatrixPart {
    pub position: Position,
    pub searched_char: char,
}

struct SearchMatrix {
    parts: Vec<SearchMatrixPart>,
    width: usize,
    height: usize,
}

impl SearchMatrix {
    pub fn new(parts: Vec<SearchMatrixPart>) -> SearchMatrix {
        let height = parts.iter().map(|part| part.position.y).max().unwrap();
        let width = parts.iter().map(|part| part.position.x).max().unwrap();

        Self {
            parts,
            width,
            height
        }
    }

    pub fn search(&self, search_space: &Vec<Vec<char>>) -> u32 {
        let search_width = search_space[0].len() - self.width;
        let search_height = search_space.len() - self.height;

        let mut num_found = 0;

        for x in 0..search_width {
            for y in 0..search_height {
                let found = self.parts.iter()
                    .fold(true, |acc, part| acc && search_space[y + part.position.y][x + part.position.x] == part.searched_char);
                if found {
                    num_found += 1;
                }
            }
        }
        num_found
    }
}


struct Day4 {}

impl Day for Day4 {
    const INPUT_FILE: &'static str = "day4.txt";
    type OutputType = u32;
    type ParsedType = Vec<Vec<char>>;

    fn parse_input(input: &str) -> Self::ParsedType {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        let matrices = vec![
            // HORIZONTAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'X' },
                SearchMatrixPart{ position: Position::new(1, 0), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(2, 0), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(3, 0), searched_char: 'S' },
            ]),
            // VERTICAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'X' },
                SearchMatrixPart{ position: Position::new(0, 1), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(0, 2), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(0, 3), searched_char: 'S' },
            ]),
            // LEFT TO RIGHT DIAGONAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'X' },
                SearchMatrixPart{ position: Position::new(1, 1), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(2, 2), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(3, 3), searched_char: 'S' },
            ]),
            // RIGHT TO LEFT DIAGONAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(3, 0), searched_char: 'X' },
                SearchMatrixPart{ position: Position::new(2, 1), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(1, 2), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(0, 3), searched_char: 'S' },
            ]),
            // BACKWARDS HORIZONTAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(1, 0), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(2, 0), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(3, 0), searched_char: 'X' },
            ]),
            // BACKWARDS VERTICAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(0, 1), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(0, 2), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(0, 3), searched_char: 'X' },
            ]),
            // BACKWARDS LEFT TO RIGHT DIAGONAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(1, 1), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(2, 2), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(3, 3), searched_char: 'X' },
            ]),
            // BACKWARDS RIGHT TO LEFT DIAGONAL
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(3, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(2, 1), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(1, 2), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(0, 3), searched_char: 'X' },
            ]),
        ];
        matrices.iter().fold(0, |acc, matrix| acc + matrix.search(input))
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
        let matrices = vec![
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(1, 1), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(2, 2), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(2, 0), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(0, 2), searched_char: 'S' },
            ]),
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(1, 1), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(2, 2), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(2, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(0, 2), searched_char: 'M' },
            ]),
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(1, 1), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(2, 2), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(2, 0), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(0, 2), searched_char: 'S' },
            ]),
            SearchMatrix::new(vec![
                SearchMatrixPart{ position: Position::new(0, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(1, 1), searched_char: 'A' },
                SearchMatrixPart{ position: Position::new(2, 2), searched_char: 'M' },
                SearchMatrixPart{ position: Position::new(2, 0), searched_char: 'S' },
                SearchMatrixPart{ position: Position::new(0, 2), searched_char: 'M' },
            ]),
        ];
        matrices.iter().fold(0, |acc, matrix| acc + matrix.search(input))
    }
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let input = Day4::parse_input(INPUT);
        assert_eq!(Day4::part1(&input), 18);
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let input = Day4::parse_input(INPUT);
        assert_eq!(Day4::part2(&input), 9);
    }
}
