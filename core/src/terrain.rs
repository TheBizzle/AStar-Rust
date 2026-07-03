#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
  Ant,
  Empty,
  Food,
  Goal,
  Mound,
  Myself,
  Path,
  Query,
  Wall,
  Water,
}

use Terrain::{Ant, Empty, Food, Goal, Mound, Myself, Path, Query, Wall, Water};

impl Terrain {
  #[must_use]
  pub const fn is_passable(&self) -> bool {
    matches!(self, Ant | Empty | Food | Goal | Mound)
  }

  #[must_use]
  pub const fn to_char(&self) -> char {
    match self {
      Ant => 'a',
      Empty => ' ',
      Food => 'f',
      Goal => 'G',
      Mound => 'O',
      Myself => '*',
      Path => 'x',
      Query => '.',
      Wall => 'D',
      Water => '%',
    }
  }

  #[must_use]
  pub fn from_char(c: char) -> Self {
    match c {
      'a' => Ant,
      ' ' => Empty,
      'f' => Food,
      'G' => Goal,
      'O' => Mound,
      '*' => Myself,
      'x' => Path,
      '.' => Query,
      'D' => Wall,
      '%' => Water,
      x => panic!("Impossible terrain (Terrain.from_char): {x}"),
    }
  }
}
