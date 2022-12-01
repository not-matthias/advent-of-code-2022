

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    let inputs = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.parse::<u32>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    let mut elves = vec![];
    let mut current_elf = 0;
    for input in inputs {
        match input {
            None => {
                elves.push(current_elf);
                current_elf = 0;
            }
            Some(value) => current_elf += value,
        };
    }

    elves
}

#[aoc(day1, part1)]
fn part1(elves: &[u32]) -> u32 { *elves.iter().max().unwrap() }

#[aoc(day1, part2)]
fn part2(elves: &[u32]) -> u32 {
    let mut elves = elves.to_vec();

    elves.sort();
    elves.into_iter().rev().take(3).sum::<u32>()
}
