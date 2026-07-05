// First version created by Claude Sonnet 5

#[cfg(test)]
mod tests {
  use core::coordinate::Coordinate;
  use core::interpreter::{PathingGrid, PathingMapString};
  use core::terrain::Terrain::{self, Empty, Goal, Mound, Myself, Wall, Water};

  const M5X5P1: &str = "DDDDD|DG  D|D   D|D  *D|DDDDD";
  const M5X5P2: &str = " DGD | DDD |%%%% |DD %%|*D  %";
  const TRBLMKR: &str = " %  *|OG% %|%    |";

  #[test]
  fn test_simple_grid() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "Simple grid",
      "*G",
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 1, y: 0 },
      Coordinate { x: 1, y: 0 },
      &[Myself, Goal],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_one_line_grid_1() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "One-line grid 1",
      "*      G",
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 7, y: 0 },
      Coordinate { x: 7, y: 0 },
      &[Myself, Empty, Empty, Empty, Empty, Empty, Empty, Goal],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_one_line_grid_2() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "One-line grid 2",
      "G      *",
      Coordinate { x: 7, y: 0 },
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 7, y: 0 },
      &[Goal, Empty, Empty, Empty, Empty, Empty, Empty, Myself],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_one_line_grid_3() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "One-line grid 3",
      "G %D%  *",
      Coordinate { x: 7, y: 0 },
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 7, y: 0 },
      &[Goal, Empty, Water, Wall, Water, Empty, Empty, Myself],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_simple_vertical_grid() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "Simple vertical grid",
      "*|G",
      Coordinate { x: 0, y: 1 },
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 0, y: 1 },
      &[Goal, Myself],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_one_line_vert_grid_1() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "One-line vert grid 1",
      "*| | | |G",
      Coordinate { x: 0, y: 4 },
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 0, y: 4 },
      &[Goal, Empty, Empty, Empty, Myself],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_one_line_vert_grid_2() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "One-line vert grid 2",
      "G| | | |*",
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 0, y: 4 },
      Coordinate { x: 0, y: 4 },
      &[Myself, Empty, Empty, Empty, Goal],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_one_line_vert_grid_3() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "One-line vert grid 3",
      "G| |%|D|*",
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 0, y: 4 },
      Coordinate { x: 0, y: 4 },
      &[Myself, Wall, Water, Empty, Goal],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_2x2_grid_1() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "2x2 grid 1",
      "G | *",
      Coordinate { x: 1, y: 0 },
      Coordinate { x: 0, y: 1 },
      Coordinate { x: 1, y: 1 },
      &[Empty, Goal, Myself, Empty],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_2x2_grid_2() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "2x2 grid 2",
      "G*|  ",
      Coordinate { x: 1, y: 1 },
      Coordinate { x: 0, y: 1 },
      Coordinate { x: 1, y: 1 },
      &[Empty, Goal, Empty, Myself],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_2x2_grid_3() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "2x2 grid 3",
      "G*|DD",
      Coordinate { x: 1, y: 1 },
      Coordinate { x: 0, y: 1 },
      Coordinate { x: 1, y: 1 },
      &[Wall, Goal, Wall, Myself],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_2x2_grid_4() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "2x2 grid 4",
      "DD|*G",
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 1, y: 0 },
      Coordinate { x: 1, y: 1 },
      &[Myself, Wall, Goal, Wall],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_5x5_grid_1() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "5x5 grid 1",
      M5X5P1,
      Coordinate { x: 3, y: 1 },
      Coordinate { x: 1, y: 3 },
      Coordinate { x: 4, y: 4 },
      &[
        Wall, Wall, Wall, Wall, Wall, Wall, Empty, Empty, Goal, Wall, Wall, Empty, Empty, Empty, Wall, Wall,
        Myself, Empty, Empty, Wall, Wall, Wall, Wall, Wall, Wall,
      ],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_5x5_grid_2() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "5x5 grid 2",
      M5X5P2,
      Coordinate { x: 0, y: 0 },
      Coordinate { x: 2, y: 4 },
      Coordinate { x: 4, y: 4 },
      &[
        Myself, Wall, Water, Empty, Empty, Wall, Wall, Water, Wall, Wall, Empty, Empty, Water, Wall, Goal,
        Empty, Water, Water, Wall, Wall, Water, Water, Empty, Empty, Empty,
      ],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  #[test]
  fn test_past_troublemaker() {
    let mut failures: Vec<String> = Vec::new();
    run_case(
      &mut failures,
      "Past troublemaker",
      TRBLMKR,
      Coordinate { x: 4, y: 2 },
      Coordinate { x: 1, y: 1 },
      Coordinate { x: 4, y: 2 },
      &[
        Water, Mound, Empty, Empty, Goal, Water, Empty, Water, Empty, Empty, Empty, Empty, Empty, Water,
        Myself,
      ],
    );
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
  }

  fn run_case(
    failures: &mut Vec<String>,
    desc: &str,
    str_grid: &str,
    start: Coordinate,
    goal: Coordinate,
    max_coord: Coordinate,
    terrains: &[Terrain],
  ) {
    let pms = PathingMapString { contents: str_grid.to_string(), delim: "|".to_string() };
    let actual = pms.as_pmd();

    if actual.start != start {
      failures.push(format!("{desc} | Start: expected {:?}, got {:?}", start, actual.start));
    }
    if actual.goal != goal {
      failures.push(format!("{desc} | Goal: expected {:?}, got {:?}", goal, actual.goal));
    }

    let expected_grid = grid_from_list(max_coord, terrains);
    let actual_grid = grid_to_sorted_vec(&actual.grid);
    if actual_grid != expected_grid {
      failures.push(format!(
        "\
{desc} | Grid mismatch:
  expected: {expected_grid:?}
  actual:   {actual_grid:?}"
      ));
    }
  }

  fn grid_to_sorted_vec(grid: &PathingGrid) -> Vec<(Coordinate, Terrain)> {
    let mut pairs: Vec<(Coordinate, Terrain)> = grid.map.iter().map(|(&c, &t)| (c, t)).collect();
    pairs.sort_by_key(|x| x.0);
    pairs
  }

  fn grid_from_list(max_coord: Coordinate, terrains: &[Terrain]) -> Vec<(Coordinate, Terrain)> {
    let mut pairs = Vec::with_capacity(terrains.len());
    let mut i = 0;
    for x in 0..=max_coord.x {
      for y in 0..=max_coord.y {
        pairs.push((Coordinate { x, y }, terrains[i]));
        i += 1;
      }
    }
    pairs.sort_by_key(|x| x.0);
    pairs
  }
}
