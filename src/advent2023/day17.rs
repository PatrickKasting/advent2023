use std::ops::RangeInclusive;

use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::{
    data_structures::grid::{Direction, Grid, Position},
    search::cheapest_path_cost,
};

type Move = (Option<Direction>, Position);

pub fn first(input: &str) -> String {
    minimum_heat_loss(&Grid::from(input), 1..=3).to_string()
}

pub fn second(input: &str) -> String {
    minimum_heat_loss(&Grid::from(input), 4..=10).to_string()
}

fn minimum_heat_loss(map: &Grid<usize>, number_of_steps: RangeInclusive<usize>) -> usize {
    let starting_point = (None, Position::new(0, 0));
    let machine_parts_factory = Position::new(map.height() - 1, map.width() - 1);
    let is_machine_parts_factory = |(_, position)| position == machine_parts_factory;
    cheapest_path_cost(
        starting_point,
        |mov| moves(map, mov, number_of_steps.clone()),
        is_machine_parts_factory,
    )
    .expect("search should reach the machine parts factory")
}

fn moves(
    map: &Grid<usize>,
    (previous_direction, position): Move,
    number_of_steps: RangeInclusive<usize>,
) -> impl Iterator<Item = (Move, usize)> + '_ {
    let next_directions = match previous_direction {
        Some(previous_direction) => vec![previous_direction.left(), previous_direction.right()],
        None => Direction::iter().collect_vec(),
    };
    next_directions.into_iter().flat_map(move |next_direction| {
        moves_in_direction(map, position, next_direction, number_of_steps.clone())
    })
}

fn moves_in_direction(
    map: &Grid<usize>,
    mut position: Position,
    direction: Direction,
    number_of_steps: RangeInclusive<usize>,
) -> Vec<(Move, usize)> {
    let mut moves = vec![];
    let mut heat_loss = 0;
    for current_number_of_steps in 1..=*number_of_steps.end() {
        position = position.neighbor(direction);
        let Some(&heat_loss_at_position) = map.get(position) else {
            break;
        };
        heat_loss += heat_loss_at_position;
        if number_of_steps.contains(&current_number_of_steps) {
            moves.push(((Some(direction), position), heat_loss));
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    use super::super::tests::test_on_input;
    use crate::{Input, Puzzle};

    const DAY: usize = 17;

    #[test]
    fn first_example() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 102);
    }

    #[test]
    fn first_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 953);
    }

    #[test]
    fn second_examples() {
        test_on_input(DAY, Puzzle::Second, Input::Example(0), 94);
        test_on_input(DAY, Puzzle::Second, Input::Example(1), 71);
    }

    #[test]
    fn second_input() {
        test_on_input(DAY, Puzzle::Second, Input::PuzzleInput, 1180);
    }
}
