#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
  North,
  East,
  South,
  West,
}

use Direction::{East, North, South, West};

impl Direction {
  pub const ALL: [Self; 4] = [North, East, South, West];
}
