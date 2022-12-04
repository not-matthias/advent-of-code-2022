use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug)]
pub struct Elf {
    pub start: usize,
    pub end: usize,
}

impl Elf {
    pub fn fully_contains(&self, other: &Elf) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl FromStr for Elf {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();

        Ok(Elf { start, end })
    }
}

pub struct Pair {
    pub elf1: Elf,
    pub elf2: Elf,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let elf1 = parts.next().unwrap().parse().unwrap();
        let elf2 = parts.next().unwrap().parse().unwrap();

        Ok(Pair { elf1, elf2 })
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .filter_map(|line| Pair::from_str(line).ok())
        .collect::<Vec<_>>()
}

#[aoc(day4, part1)]
fn solve_part_1(input: &[Pair]) -> u32 {
    let mut count = 0;
    for pair in input {
        if pair.elf1.fully_contains(&pair.elf2)
            || pair.elf2.fully_contains(&pair.elf1)
        {
            count += 1;
        }
    }

    count
}

#[aoc(day4, part2)]
fn solve_part_2(input: &[Pair]) -> u32 {
    //
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
    }

    #[test]
    fn test_example() {
        let input = get_input();
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), 2);
        assert_eq!(solve_part_2(&input), 0);
    }
}
