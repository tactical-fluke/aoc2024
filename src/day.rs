use std::fs;

pub trait Day {
    const INPUT_FILE: &'static str;
    type OutputType;
    type ParsedType;

    fn parse_input(input: &str) -> Self::ParsedType;
    fn part1(input: &Self::ParsedType) -> Self::OutputType;
    fn part2(input: &Self::ParsedType) -> Self::OutputType;
}

pub fn run_day<D: Day>() -> (D::OutputType, D::OutputType) {
    let input = fs::read_to_string(format!("inputs/{}", D::INPUT_FILE)).unwrap();
    let input = D::parse_input(&input);
    (D::part1(&input), D::part2(&input))
}
