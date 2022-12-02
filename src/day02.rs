use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub enum RoundResult {
    Loss,
    Draw,
    Win,
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    // Finds the correct move to achieve the round result (win, draw, loss)
    pub fn correct_move(&self, result: RoundResult) -> Shape {
        match result {
            RoundResult::Win => match self {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            RoundResult::Draw => *self,
            RoundResult::Loss => match self {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
        }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" | "A" => Ok(Shape::Rock),
            "Y" | "B" => Ok(Shape::Paper),
            "Z" | "C" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

pub struct Part1Move {
    player1: Shape,
    player2: Shape,
}

impl Part1Move {
    pub fn play(&self) -> (u32, u32) {
        let (a, b) = match (self.player1, self.player2) {
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => (3, 3),
            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => (0, 6),
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => (6, 0),
        };

        (a + self.player1.score(), b + self.player2.score())
    }
}

impl FromStr for Part1Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let player1 = parts.next().ok_or(())?.parse()?;
        let player2 = parts.next().ok_or(())?.parse()?;
        Ok(Part1Move { player1, player2 })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Part2Move {
    player1: Shape,
    result: RoundResult,
}

impl Part2Move {
    pub fn play(&self) -> (u32, u32) {
        let player2 = self.player1.correct_move(self.result);

        Part1Move {
            player1: self.player1,
            player2,
        }
        .play()
    }
}

impl FromStr for Part2Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let player1 = parts.next().ok_or(())?.parse()?;
        let result = parts.next().ok_or(())?.parse()?;
        Ok(Part2Move { player1, result })
    }
}

#[aoc_generator(day2, part1)]
fn part_1_parse_input(input: &str) -> Vec<Part1Move> {
    input
        .lines()
        .map(|line| Part1Move::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

#[aoc_generator(day2, part2)]
fn part_2_parse_input(input: &str) -> Vec<Part2Move> {
    input
        .lines()
        .map(|line| Part2Move::from_str(line).unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
fn solve_part_1(input: &[Part1Move]) -> u32 {
    input.iter().map(|m| m.play().1).sum::<u32>()
}

#[aoc(day2, part2)]
fn solve_part_2(input: &[Part2Move]) -> u32 {
    input.iter().map(|m| m.play().1).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> &'static str { "A Y\nB X\nC Z" }

    #[test]
    fn test_example() {
        let input = get_input();

        assert_eq!(solve_part_1(&part_1_parse_input(input)), 15);
        assert_eq!(solve_part_2(&part_2_parse_input(input)), 12);
    }
}
