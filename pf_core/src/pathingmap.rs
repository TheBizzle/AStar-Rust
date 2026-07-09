use super::coordinate::Coordinate;
use super::interpreter::PathingGrid;
use super::terrain::Terrain::{self, Goal, Myself, Path, Query};

#[derive(Debug)]
pub struct PathingMap {
  pub grid: PathingGrid,
}

impl PathingMap {
  #[must_use]
  pub fn height(&self) -> usize {
    self.max_coord().y + 1
  }

  #[must_use]
  pub fn width(&self) -> usize {
    self.max_coord().x + 1
  }

  pub fn insert_path(&mut self, coords: Vec<Coordinate>) {
    for coord in coords {
      if let Some(&terrain) = self.grid.map.get(&coord) {
        if terrain != Myself && terrain != Goal {
          self.grid.map.insert(coord, Path);
        }
      } else {
        panic!("{coord:?} is not a valid coordinate!");
      }
    }
  }

  pub fn insert_query(&mut self, coords: Vec<Coordinate>) {
    for coord in coords {
      if let Some(&terrain) = self.grid.map.get(&coord) {
        if terrain != Myself && terrain != Goal {
          self.grid.map.insert(coord, Query);
        }
      } else {
        panic!("{coord:?} is not a valid coordinate!");
      }
    }
  }

  pub fn mark_as_goal(&mut self, coord: Coordinate) {
    self.grid.map.insert(coord, Goal);
  }

  #[must_use]
  pub fn neighbors_of(&self, coord: Coordinate) -> Vec<Coordinate> {
    let Coordinate { x, y } = coord;
    [
      y.checked_add(1).map(|y| Coordinate { x, y }),
      y.checked_sub(1).map(|y| Coordinate { x, y }),
      x.checked_add(1).map(|x| Coordinate { x, y }),
      x.checked_sub(1).map(|x| Coordinate { x, y }),
    ]
    .into_iter()
    .flatten()
    .filter(|c| self.grid.map.get(c).is_some_and(Terrain::is_passable))
    .collect()
  }

  pub fn step(&mut self, prev: Coordinate, next: Coordinate) {
    self.grid.map.insert(prev, Query);
    self.grid.map.insert(next, Myself);
  }

  #[must_use]
  pub fn as_string(&self) -> String {
    let mut coords: Vec<_> = self.grid.map.keys().copied().collect();
    coords.sort();

    if let Some(&max_coord) = coords.last() {
      let mut lines: Vec<String> = Vec::with_capacity(max_coord.y);

      for y in 0..=(max_coord.y) {
        let mut str = String::with_capacity(max_coord.x);
        for x in 0..=(max_coord.x) {
          let coord = Coordinate { x, y };
          let msg = format!("Coordinate {max_coord:?} does not exist!");
          let t = self.grid.map.get(&coord).expect(&msg);
          str.push(t.to_char());
        }
        lines.push(str);
      }

      // Recall that we reoriented the coordinates when reading --Jason B. (4/28/26)
      let lines_with_borders: Vec<_> = lines.iter().rev().map(|line| format!("|{line}|")).collect();
      let border = "-".repeat(max_coord.x + 1);
      let main_content = lines_with_borders.join("\n");

      format!(
        "\
+{border}+
{main_content}
+{border}+"
      )
    } else {
      String::new()
    }
  }

  fn max_coord(&self) -> &Coordinate {
    self.grid.map.keys().max().expect("All maps must have at least one coordinate")
  }
}
