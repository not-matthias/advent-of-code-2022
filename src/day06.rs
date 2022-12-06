use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

fn all_different(chunk: &[u8]) -> bool {
    let mut seen = [false; 26];
    for &c in chunk {
        let idx = (c - b'a') as usize;
        if seen[idx] {
            return false;
        }
        seen[idx] = true;
    }
    true
}

#[aoc(day6, part1)]
fn solve_part_1(input: &str) -> usize {
    for (i, chunk) in input.as_bytes().windows(4).enumerate() {
        if all_different(chunk) {
            return i + 4;
        }
    }

    unreachable!()
}

#[aoc(day6, part2)]
fn solve_part_2(input: &str) -> u32 {
    //
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        // assert_eq!(solve_part_2(input), 0);
    }
}
