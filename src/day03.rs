use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item(char);

impl Item {
    /// Returns the following:
    /// 1 = a, 26 = z
    /// 27 = A, 52 = Z
    pub fn priority(&self) -> u8 {
        match self.0 {
            'a'..='z' => self.0 as u8 - 96,
            'A'..='Z' => self.0 as u8 - 64 + 26,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rucksack {
    pub items: Vec<Item>,
}

impl Rucksack {
    pub fn new(items: Vec<Item>) -> Self { Self { items } }

    pub fn shared_items(&self) -> HashSet<Item> {
        // Panic if we have an uneven number of items
        assert_eq!(self.items.len() % 2, 0);

        let (part1, part2) = self.items.split_at(self.items.len() / 2);

        let mut shared = HashSet::new();
        for item in part2 {
            if part1.contains(item) {
                shared.insert(*item);
            }
        }

        shared
    }

    /// Checks if the item is in both compartments.
    pub fn contains_item(&self, item: &Item) -> bool {
        let (part1, part2) = self.items.split_at(self.items.len() / 2);

        part1.contains(item) && part2.contains(item)
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            Rucksack::new(line.chars().map(Item).collect::<Vec<Item>>())
        })
        .collect::<Vec<_>>()
}

#[aoc(day3, part1)]
fn solve_part_1(input: &[Rucksack]) -> u32 {
    input
        .iter()
        .flat_map(|r| r.shared_items())
        .map(|i| i.priority() as u32)
        .sum()
}

#[aoc(day3, part2)]
fn solve_part_2(input: &[Rucksack]) -> u32 {
    let mut badges = Vec::<Item>::new();
    for chunk in input.chunks(3) {
        let mut shared = HashSet::new();
        for item in &chunk[0].items {
            if chunk[1].items.contains(item) && chunk[2].items.contains(item) {
                shared.insert(item);
            }
        }

        badges.extend(shared);
    }

    badges.into_iter().map(|i| i.priority() as u32).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(Item('a').priority(), 1);
        assert_eq!(Item('z').priority(), 26);
        assert_eq!(Item('A').priority(), 27);
        assert_eq!(Item('Z').priority(), 52);
    }

    fn get_input() -> &'static str {
        r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#
    }

    #[test]
    fn test_part1() {
        let input = get_input();
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 157);
    }

    #[test]
    fn test_part2() {
        let input = r#"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg"#;

        let input = parse_input(input);

        assert_eq!(solve_part_2(&input), 70);
    }
}
