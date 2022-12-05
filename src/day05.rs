use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, str::FromStr};

#[derive(Clone, Debug)]
pub struct Move {
    from: usize,
    to: usize,
    count: usize,
}

#[derive(Clone, Debug)]
pub struct Ship {
    crates: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

impl Ship {
    pub fn top_crate_names(&self) -> String {
        self.crates.iter().filter_map(|c| c.back()).collect()
    }
}

impl FromStr for Ship {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split("\n\n");

        // Parse the stack
        //
        let lines = parts.next().unwrap().lines();
        let stack_count = (input.lines().next().unwrap().len() + 1) / 4;
        let mut crates = vec![VecDeque::new(); stack_count];

        for line in lines {
            for i in 0..stack_count {
                if line.chars().nth(i * 4 + 0) == Some('[') {
                    crates[i].push_front(line.chars().nth(i * 4 + 1).unwrap());
                }
            }
        }

        // Parse the moves
        //
        let mut moves = Vec::new();
        for line in parts.next().unwrap().lines() {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            }

            let captures = RE.captures(line).unwrap();
            moves.push(Move {
                from: captures[2].parse().unwrap(),
                to: captures[3].parse().unwrap(),
                count: captures[1].parse().unwrap(),
            });
        }

        Ok(Ship { crates, moves })
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Ship { input.parse().unwrap() }

#[aoc(day5, part1)]
fn solve_part_1(input: &Ship) -> String {
    let mut input: Ship = input.clone();

    for mv in &input.moves {
        for _ in 0..mv.count {
            if let Some(item) = input.crates[mv.from - 1].pop_back() {
                input.crates[mv.to - 1].push_back(item);
            }
        }
    }

    input.top_crate_names()
}

#[aoc(day5, part2)]
fn solve_part_2(input: &Ship) -> String {
    let mut input: Ship = input.clone();

    for mv in &input.moves {
        let mut items = VecDeque::new();
        for _ in 0..mv.count {
            if let Some(item) = input.crates[mv.from - 1].pop_back() {
                items.push_front(item);
            }
        }

        input.crates[mv.to - 1].append(&mut items);
    }

    input.top_crate_names()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str {
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#
    }

    #[test]
    fn test_example() {
        let input = get_input();
        let input = parse_input(input);

        assert_eq!(solve_part_1(&input), "CMZ");
        assert_eq!(solve_part_2(&input), "MCD");
    }
}
