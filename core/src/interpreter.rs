use std::collections::HashMap;

use super::coordinate::Coordinate;
use super::terrain::Terrain::{self, Goal, Myself};

#[derive(Debug)]
pub struct PathingGrid {
  pub map: HashMap<Coordinate, Terrain>,
}

#[derive(Debug)]
pub struct PathingMapString {
  pub contents: String,
  pub delim: String,
}

#[derive(Debug)]
pub struct PathingMapData {
  pub start: Coordinate,
  pub goal: Coordinate,
  pub grid: PathingGrid,
}

impl PathingMapString {
  pub fn as_pmd(&self) -> PathingMapData {
    let Self { contents: pms, delim: del } = self;
    if pms.is_empty() {
      panic!("Cannot build map from empty string!")
    } else {
      type VS = Vec<String>;
      let delim = del.as_str();
      let strs: VS = pms.strip_suffix(delim).unwrap_or(pms).split(delim).map(ToString::to_string).collect();
      let grid = strs_to_grid(&strs);
      let (start, goal) = grid.find_start_and_goal();
      PathingMapData { start, goal, grid }
    }
  }
}

impl PathingGrid {
  fn find_start_and_goal(&self) -> (Coordinate, Coordinate) {
    let mut start_opt: Option<Coordinate> = None;
    let mut end_opt: Option<Coordinate> = None;
    for (&key, &value) in &self.map {
      match value {
        Myself => start_opt = Some(key),
        Goal => end_opt = Some(key),
        _ => {},
      }
    }
    start_opt.zip(end_opt).expect("Pathing map must have both start and goal!")
  }
}

// We start with rows of strings (strs[a][b] => a: 0 = top row, b: 0 = leftmost character) and we need to
// transform it such that it follows normal Cartesian coordinate rules (strs[a][b] => a: 0 = leftmost
// character, b: 0 = bottom row) --Jason B. (7/2/26)
fn strs_to_grid(strs: &[String]) -> PathingGrid {
  let mut map = HashMap::new();
  let rotated = rotate_clockwise(strs);
  for (x, col) in rotated.iter().enumerate() {
    for (y, val) in col.chars().enumerate() {
      map.insert(Coordinate { x, y }, Terrain::from_char(val));
    }
  }
  PathingGrid { map }
}

fn rotate_clockwise(rows: &[String]) -> Vec<String> {
  if rows.is_empty() {
    Vec::new()
  } else {
    (0..rows[0].chars().count())
      .map(|col| rows.iter().rev().map(|r| r.chars().nth(col).unwrap()).collect::<String>())
      .collect()
  }
}
