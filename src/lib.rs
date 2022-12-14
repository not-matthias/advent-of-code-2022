#[macro_use] extern crate aoc_runner_derive;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;

#[cfg(target_env = "DONT_COMPILE_THIS")] pub mod template;

aoc_lib! { year = 2022}
