use pf_core::coordinate::Coordinate;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Heuristic {
  Euclidean,
  Manhattan,
  Dijkstra,
}

use Heuristic::{Dijkstra, Euclidean, Manhattan};

impl Heuristic {
  #[allow(clippy::cast_precision_loss)]
  #[must_use]
  pub fn eval(self, c1: Coordinate, c2: Coordinate) -> f64 {
    match self {
      Euclidean => {
        let x_comp = c1.x.abs_diff(c2.x) as u64;
        let y_comp = c1.y.abs_diff(c2.y) as u64;
        let comps = (x_comp * x_comp) + (y_comp * y_comp);
        (comps as f64).sqrt()
      },
      Manhattan => (c1.x.abs_diff(c2.x) + c1.y.abs_diff(c2.y)) as f64,
      Dijkstra => 0.0,
    }
  }
}
