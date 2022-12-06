use aoc_runner_derive::aoc;

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

fn find_marker(input: &str, packet_size: usize) -> usize {
    for (i, chunk) in input.as_bytes().windows(packet_size).enumerate() {
        if all_different(chunk) {
            return i + packet_size;
        }
    }

    unreachable!();
}

#[aoc(day6, part1)]
fn solve_part_1(input: &str) -> usize { find_marker(input, 4) }

#[aoc(day6, part2)]
fn solve_part_2(input: &str) -> usize { find_marker(input, 14) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(solve_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
