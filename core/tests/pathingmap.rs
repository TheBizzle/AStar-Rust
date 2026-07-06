// First version created by Claude Sonnet 5
#[cfg(test)]
mod tests {
  use core::coordinate::Coordinate;
  use core::interpreter::PathingMapString;
  use core::pathingmap::PathingMap;

  fn grid_from_string(s: &str) -> PathingMap {
    let pms = PathingMapString { contents: s.into(), delim: "|".into() };
    PathingMap { grid: pms.as_pmd().grid }
  }

  fn base_map() -> PathingMap {
    let pms = PathingMapString { contents: " DGD | DDD |%%%% |DD %%|*D  %".into(), delim: "|".into() };
    PathingMap { grid: pms.as_pmd().grid }
  }

  #[test]
  fn test_neighbors_of_1() {
    test_neighbors("neighborsOf 1", Coordinate { x: 9001, y: 9001 }, &[]);
  }

  #[test]
  fn test_neighbors_of_2() {
    test_neighbors(
      "neighborsOf 2",
      Coordinate { x: 2, y: 0 },
      &[Coordinate { x: 2, y: 1 }, Coordinate { x: 3, y: 0 }],
    );
  }

  #[test]
  fn test_neighbors_of_3() {
    test_neighbors("neighborsOf 3", Coordinate { x: 3, y: 0 }, &[Coordinate { x: 2, y: 0 }]);
  }

  #[test]
  fn test_step() {
    let mut base = base_map();
    base.step(Coordinate { x: 2, y: 0 }, Coordinate { x: 2, y: 1 });
    test_map("step", &base, &grid_from_string(" DGD | DDD |%%%% |DD*%%|*D. %"));
  }

  #[test]
  fn test_mark_as_goal_1() {
    let mut base = base_map();
    base.mark_as_goal(Coordinate { x: 2, y: 0 });
    test_map("markAsGoal 1", &base, &grid_from_string(" DGD | DDD |%%%% |DD %%|*DG %"));
  }

  #[test]
  fn test_mark_as_goal_2() {
    let mut base = base_map();
    base.mark_as_goal(Coordinate { x: 2, y: 1 });
    test_map("markAsGoal 2", &base, &grid_from_string(" DGD | DDD |%%%% |DDG%%|*D  %"));
  }

  #[test]
  fn test_insert_path_1() {
    let mut base = base_map();
    base.insert_path(vec![Coordinate { x: 4, y: 3 }, Coordinate { x: 4, y: 2 }]);
    test_map("insertPath 1", &base, &grid_from_string(" DGD | DDDx|%%%%x|DD %%|*D  %"));
  }

  #[test]
  fn test_insert_path_2() {
    let mut base = base_map();
    base.insert_path(vec![Coordinate { x: 2, y: 1 }, Coordinate { x: 2, y: 0 }, Coordinate { x: 3, y: 0 }]);
    test_map("insertPath 2", &base, &grid_from_string(" DGD | DDD |%%%% |DDx%%|*Dxx%"));
  }

  #[test]
  fn test_insert_path_3() {
    let mut base = base_map();
    base.insert_path(Vec::new());
    test_map("insertPath 3", &base, &grid_from_string(" DGD | DDD |%%%% |DD %%|*D  %"));
  }

  fn test_map(desc: &str, actual: &PathingMap, expected: &PathingMap) {
    assert_eq!(actual.as_string(), expected.as_string(), "{desc}");
  }

  fn test_neighbors(desc: &str, coord: Coordinate, expected: &[Coordinate]) {
    let actual = &base_map().neighbors_of(coord);
    assert_eq!(actual, expected, "{desc}");
  }
}
