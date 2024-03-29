use std::ops::{Add, Sub};

use itertools::Itertools;

use crate::strings::isizes;

type Number = isize;

#[derive(Debug, Clone, Copy)]
struct Extrapolation {
    combination: fn(Number, Number) -> Number,
    prediction: fn(Number, Number) -> Number,
}

fn row(preceding_row: &[Number], combination: fn(Number, Number) -> Number) -> Vec<Number> {
    preceding_row
        .windows(2)
        .map(|pair| combination(pair[0], pair[1]))
        .collect_vec()
}

fn extrapolation(history: &[Number], extrapolation: Extrapolation) -> Number {
    if history.iter().all(|&number| number == 0) {
        return 0;
    }
    let &last_number = history.last().expect("history should not be empty");
    let succeeding_row = row(history, extrapolation.combination);
    let succeeding_prediction = self::extrapolation(&succeeding_row, extrapolation);
    (extrapolation.prediction)(last_number, succeeding_prediction)
}

fn prediction(history: &str, reverse: bool) -> Number {
    let mut history = isizes(history);
    if reverse {
        history.reverse();
    }

    let extrapolation = if reverse {
        Extrapolation {
            combination: Number::sub,
            prediction: Number::sub,
        }
    } else {
        Extrapolation {
            combination: |left, right| right - left,
            prediction: Number::add,
        }
    };

    self::extrapolation(&history, extrapolation)
}

fn sum_of_predictions(input: &str, reverse: bool) -> Number {
    input
        .lines()
        .map(|history| prediction(history, reverse))
        .sum()
}

pub fn first(input: &str) -> String {
    sum_of_predictions(input, false).to_string()
}

pub fn second(input: &str) -> String {
    sum_of_predictions(input, true).to_string()
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 9;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 114);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_995_001_648);
    }

    #[test]
    fn second_example() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 2);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 988);
    }
}
