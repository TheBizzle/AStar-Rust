use std::cmp::Ordering::{self, Equal};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coordinate {
  pub x: usize,
  pub y: usize,
}

impl Ord for Coordinate {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.y.cmp(&other.y) {
      Equal => self.x.cmp(&other.x),
      res => res,
    }
  }
}

impl PartialOrd for Coordinate {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Debug)]
pub enum Breadcrumb {
  Source { coord: Coordinate },
  Crumb { to: Coordinate, from: Box<Self> },
}

impl Breadcrumb {
  #[must_use]
  pub fn to_sequence(&self) -> Vec<Coordinate> {
    use Breadcrumb::{Crumb, Source};
    match self {
      Source { coord } => vec![*coord],
      Crumb { to, from } => {
        let mut v = from.to_sequence();
        v.push(*to);
        v
      },
    }
  }
}
