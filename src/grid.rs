#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use std::{
    collections::HashSet,
    convert::identity,
    fmt::{Debug, Display, Write},
    ops::{Index, IndexMut},
};

use itertools::Itertools;
use strum::{EnumIter, IntoEnumIterator};

use crate::utilities::as_isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn next_clockwise(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    pub fn opposite(self) -> Self {
        self.next_clockwise().next_clockwise()
    }

    pub fn next_counterclockwise(self) -> Direction {
        self.opposite().next_clockwise()
    }

    pub fn reflection_north_west_diagonal(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::North,
            Direction::South => Direction::East,
            Direction::East => Direction::South,
        }
    }

    pub fn reflection_north_east_diagonal(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::West => Direction::South,
            Direction::South => Direction::West,
            Direction::East => Direction::North,
        }
    }

    pub fn as_unit_vector(self) -> Position {
        Position::new(0, 0).neighbor(self)
    }
}

impl TryFrom<(Position, Position)> for Direction {
    type Error = &'static str;

    fn try_from((from, to): (Position, Position)) -> Result<Self, Self::Error> {
        if from == to {
            return Err("positions should not be identical");
        }

        if from.row() == to.row() {
            if from.column() < to.column() {
                Ok(Direction::East)
            } else {
                Ok(Direction::West)
            }
        } else if from.column() == to.column() {
            if from.row() < to.row() {
                Ok(Direction::South)
            } else {
                Ok(Direction::North)
            }
        } else {
            Err("positions should share a row or a column")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Curvature {
    Straight,
    LeftTurn,
    UTurn,
    RightTurn,
}

impl Curvature {
    pub fn as_seen_from_opposite_direction(self) -> Curvature {
        match self {
            Curvature::Straight => Curvature::Straight,
            Curvature::LeftTurn => Curvature::RightTurn,
            Curvature::UTurn => Curvature::UTurn,
            Curvature::RightTurn => Curvature::LeftTurn,
        }
    }
}

impl From<(Direction, Direction)> for Curvature {
    fn from((towards, away): (Direction, Direction)) -> Self {
        if towards == away {
            Self::Straight
        } else if towards == away.next_clockwise() {
            Self::LeftTurn
        } else if towards.opposite() == away {
            Self::UTurn
        } else {
            Self::RightTurn
        }
    }
}

pub type Coordinate = isize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    row: Coordinate,
    column: Coordinate,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl Position {
    pub fn new(row: Coordinate, column: Coordinate) -> Self {
        Position { row, column }
    }

    pub fn row(self) -> Coordinate {
        self.row
    }

    pub fn column(self) -> Coordinate {
        self.column
    }

    pub fn set_row(&mut self, coordinate: Coordinate) {
        self.row = coordinate;
    }

    pub fn set_column(&mut self, coordinate: Coordinate) {
        self.column = coordinate;
    }

    pub fn neighbor(mut self, direction: Direction) -> Position {
        match direction {
            Direction::North => self.row -= 1,
            Direction::East => self.column += 1,
            Direction::South => self.row += 1,
            Direction::West => self.column -= 1,
        }
        self
    }

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        Direction::iter().map(move |direction| self.neighbor(direction))
    }

    pub fn addition(mut self, other: Position) -> Position {
        self.row += other.row;
        self.column += other.column;
        self
    }

    pub fn scalar_product(mut self, scalar: Coordinate) -> Position {
        self.row *= scalar;
        self.column *= scalar;
        self
    }

    pub fn dot_product(self, other: Position) -> Coordinate {
        self.row * other.row + self.column * other.column
    }

    fn coordinates_as_usize(self) -> [Option<usize>; 2] {
        [self.row, self.column]
            .map(|coordinate| (!coordinate.is_negative()).then_some(coordinate as usize))
    }
}

fn is_clockwise(cycle: &[Position]) -> bool {
    let curvature_counts = cycle
        .iter()
        .copied()
        .circular_tuple_windows::<(Position, Position, Position)>()
        .map(|(first, second, third)| {
            let [towards, away] = [
                Direction::try_from((first, second)),
                Direction::try_from((second, third)),
            ]
            .map(|direction| direction.expect("positions should be neighbors"));
            Curvature::from((towards, away))
        })
        .counts();
    let difference = as_isize(curvature_counts[&Curvature::RightTurn])
        - as_isize(curvature_counts[&Curvature::LeftTurn]);
    debug_assert_eq!(
        difference.abs(),
        4,
        "turn count difference should be four or negative four",
    );
    difference.is_positive()
}

pub fn area(cycle: &mut [Position]) -> HashSet<Position> {
    if !is_clockwise(cycle) {
        cycle.reverse();
    }

    let cycle_as_hash_set: HashSet<Position> = cycle.iter().copied().collect();
    let mut area = HashSet::new();
    let mut frontier = Vec::new();
    for (&first, &second, &third) in cycle.iter().circular_tuple_windows() {
        let [towards, away] = [
            Direction::try_from((first, second)),
            Direction::try_from((second, third)),
        ]
        .map(|direction| direction.expect("positions should be neighbors"));
        let directions_towards_area = match Curvature::from((towards, away)) {
            Curvature::UTurn => vec![
                away.next_clockwise(),
                away.opposite(),
                away.next_counterclockwise(),
            ],
            Curvature::LeftTurn => vec![away.next_clockwise(), away.opposite()],
            Curvature::Straight => vec![away.next_clockwise()],
            Curvature::RightTurn => vec![],
        };
        for direction in directions_towards_area {
            frontier.push(second.neighbor(direction));
        }

        while let Some(position) = frontier.pop() {
            if !cycle_as_hash_set.contains(&position) && area.insert(position) {
                frontier.extend(position.neighbors());
            }
        }
    }
    area
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    fn from_str(grid: &str, mut element_from_char: impl FnMut(char) -> T) -> Self {
        let elements = grid
            .lines()
            .map(|line| line.chars().map(&mut element_from_char).collect_vec())
            .collect_vec();
        Self(elements)
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        let [row, column] = position.coordinates_as_usize();
        self.0.get(row?)?.get(column?)
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        let [row, column] = position.coordinates_as_usize();
        self.0.get_mut(row?)?.get_mut(column?)
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.rows().enumerate().flat_map(|(row_index, row)| {
            row.enumerate().map(move |(column_index, element)| {
                (
                    Position::new(row_index as isize, column_index as isize),
                    element,
                )
            })
        })
    }

    pub fn iter_column_major(&self) -> impl Iterator<Item = (Position, &T)> {
        self.columns()
            .enumerate()
            .flat_map(|(column_index, column)| {
                column.enumerate().map(move |(row_index, element)| {
                    (
                        Position::new(row_index as isize, column_index as isize),
                        element,
                    )
                })
            })
    }

    pub fn rows(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        self.0.iter().map(|row| row.iter())
    }

    pub fn columns(
        &self,
    ) -> impl ExactSizeIterator<Item = impl Iterator<Item = &T>> + DoubleEndedIterator {
        (0..self.width())
            .map(|column_index| self.0.iter().map(move |row| &row[column_index as usize]))
    }

    pub fn height(&self) -> Coordinate {
        self.0.len() as Coordinate
    }

    pub fn width(&self) -> Coordinate {
        self.0
            .first()
            .map(|row| row.len() as Coordinate)
            .unwrap_or_default()
    }
}

impl<S: AsRef<str>> From<S> for Grid<char> {
    fn from(grid: S) -> Self {
        Self::from_str(grid.as_ref(), identity)
    }
}

impl<S: AsRef<str>> From<S> for Grid<usize> {
    fn from(grid: S) -> Self {
        Self::from_str(grid.as_ref(), |char| char as usize - '0' as usize)
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for &element in row {
                f.write_char(element)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, position: Position) -> &Self::Output {
        self.get(position).expect("position should be within grid")
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        self.get_mut(position)
            .expect("position should be within grid")
    }
}
