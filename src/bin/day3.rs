use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day3>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

struct Day3 {}

impl Day for Day3 {
    const INPUT_FILE: &'static str = "day3.txt";
    type OutputType = u32;
    type ParsedType = Vec<(u32, u32)>;

    fn parse_input(input: &str) -> Self::ParsedType {
        todo!()
    }

    fn part1(input: &Self::ParsedType) -> Self::OutputType {
        todo!()
    }

    fn part2(input: &Self::ParsedType) -> Self::OutputType {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
    }

    #[test]
    fn test_part2() {
    }
}
