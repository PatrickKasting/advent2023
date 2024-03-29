#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod advent2022;
mod advent2023;
mod data_structures;
mod math;
mod matrix;
mod search;
mod strings;

use std::{fmt::Debug, fs, ops::RangeInclusive};

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use strum::EnumString;

fn usize_within(range: RangeInclusive<usize>, str: &str) -> Result<usize> {
    let usize = str
        .parse()
        .map_err(|_| anyhow!("value should be a positive number"))?;
    if range.contains(&usize) {
        Ok(usize)
    } else {
        Err(anyhow!(
            "value should be between {} and {}",
            range.start(),
            range.end(),
        ))
    }
}

type Year = usize;

fn year(str: &str) -> Result<Year> {
    usize_within(2022..=2023, str)
}

type Day = usize;

fn day(str: &str) -> Result<Day> {
    usize_within(1..=25, str)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumString)]
pub enum Puzzle {
    #[strum(ascii_case_insensitive)]
    First,

    #[strum(ascii_case_insensitive)]
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Parser)]
#[command(about, long_about = None)]
struct CommandLineArguments {
    /// Which year?
    #[clap(value_parser=year)]
    year: Year,

    /// Which day?
    #[clap(value_parser=day)]
    day: Day,

    /// First or second puzzle?
    puzzle: Puzzle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Input {
    Example(usize),
    PuzzleInput,
}

fn input(year: usize, day: usize, input: Input) -> String {
    let path = match input {
        Input::Example(example) => format!("examples/{year}/{day:02}/{example}.txt"),
        Input::PuzzleInput => format!("puzzle-inputs/{year}/{day:02}.txt"),
    };
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("'{path}' should exist"))
}

type Solution = fn(&str) -> String;

fn solution(year: usize, day: usize, puzzle: Puzzle) -> Solution {
    let solution = match year {
        2022 => advent2022::solution,
        2023 => advent2023::solution,
        _ => panic!("year should be 2022 or 2023"),
    };
    solution(day, puzzle)
}

fn main() {
    let command_line_arguments = CommandLineArguments::parse();

    let input = input(
        command_line_arguments.year,
        command_line_arguments.day,
        Input::PuzzleInput,
    );
    let solution = solution(
        command_line_arguments.year,
        command_line_arguments.day,
        command_line_arguments.puzzle,
    );
    let answer = solution(&input);
    println!("{answer}");
}

#[cfg(test)]
pub mod tests {
    use itertools::Itertools;

    use super::*;

    /// # Panics
    ///
    /// Panics if the return value of the solution applied to the input does not equal
    /// `expected.to_string()`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn test_on_input(
        year: Year,
        day: Day,
        puzzle: Puzzle,
        input: Input,
        expected: impl ToString,
    ) {
        let actual = solution(year, day, puzzle)(&super::input(year, day, input));
        assert_eq!(actual, expected.to_string());
    }

    /// # Panics
    ///
    /// Panics if there is a mismatch between the return value of `function` applied to a test case
    /// from `cases` and the corresponding expected answer from `expected`. Also panics if the
    /// number of test cases and the number of expected answers differ.
    pub fn test_cases<Case, Answer: Debug + Eq>(
        function: impl FnMut(Case) -> Answer,
        cases: impl IntoIterator<Item = Case>,
        expected: impl IntoIterator<Item = Answer>,
    ) {
        for (actual, expected) in cases.into_iter().map(function).zip_eq(expected) {
            assert_eq!(actual, expected);
        }
    }
}
