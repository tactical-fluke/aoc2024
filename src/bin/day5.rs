use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::{HashMap, HashSet};
use aoc2024::day::{run_day, Day};

fn main() {
    let results = run_day::<Day5>();
    println!("part1:{}, part2:{}", results.0, results.1);
}

struct RuleSet {
    after_rules: HashMap<u32, HashSet<u32>>,
}

impl RuleSet {
    fn new() -> RuleSet {
        Self {
            after_rules: HashMap::new(),
        }
    }
}

struct Day5 {}

impl Day for Day5 {
    const INPUT_FILE: &'static str = "day5.txt";
    type OutputType = u32;
    type ParsedType = (RuleSet, Vec<Vec<u32>>);

    fn parse_input(input: &str) -> Self::ParsedType {
        let mut rule_sets = RuleSet::new();
        let mut updates = Vec::new();
        let mut parsing_rules = true;
        for line in input.lines() {
            if parsing_rules {
                if line.is_empty() {
                    parsing_rules = false;
                    continue;
                }

                let (before, after) = line.split_once("|").unwrap();
                let (before, after) = (before.parse().unwrap(), after.parse().unwrap());
                rule_sets.after_rules.entry(before).or_insert(HashSet::new()).insert(after);
            } else {
                let update = line.split(',').map(|x| x.parse().unwrap()).collect();
                updates.push(update);
            }
        }
        (rule_sets, updates)
    }

    fn part1((rule_sets, updates): &Self::ParsedType) -> Self::OutputType {
        updates.iter()
            .map(|update| (update, Self::check_update(update, &rule_sets)))
            .filter(|(_, valid)| *valid)
            .map(|(update, _)| update)
            .map(Self::get_middle_element)
            .sum()
    }

    fn part2((rule_sets, updates): &Self::ParsedType) -> Self::OutputType {
        let mut fixed_updates = Vec::with_capacity(updates.len());
        let updates = updates.clone();
        for mut update in updates {
            let valid = Self::check_update(&update, &rule_sets);
            if !valid {
                Self::fix_update(&mut update, rule_sets);
                fixed_updates.push(update)
            }
        }
        fixed_updates.iter().map(Self::get_middle_element).sum()
    }
}

impl Day5 {
    fn check_update(update: &Vec<u32>, rule_set: &RuleSet) -> bool {
        let mut before_set: HashSet<u32> = HashSet::new();

        for page in update {
            if !rule_set.after_rules.get(page).unwrap_or(&HashSet::new()).intersection(&before_set).collect::<Vec<_>>().is_empty(){
                return false;
            }
            before_set.insert(page.clone());
        }

        true
    }

    fn fix_update(update: &mut Vec<u32>, rule_set: &RuleSet) {
        update.sort_by(|a, b| {
            if rule_set.after_rules.get(a).unwrap_or(&HashSet::new()).contains(b) {
                Less
            } else if rule_set.after_rules.get(b).unwrap_or(&HashSet::new()).contains(a) {
                Greater
            } else  { Equal }
        });
    }

    fn get_middle_element(update: &Vec<u32>) -> &u32 {
        update.get(update.len() / 2).unwrap()
    }
}

#[cfg(test)]
mod day5_tests {
    use super::*;

    #[test]
    fn test_part1() {
        const TEST_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let input = Day5::parse_input(TEST_INPUT);
        let result = Day5::part1(&input);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part2() {
        const TEST_INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let input = Day5::parse_input(TEST_INPUT);
        let result = Day5::part2(&input);
        assert_eq!(result, 123);
    }
}
