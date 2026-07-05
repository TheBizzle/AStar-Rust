// First version created by Claude Sonnet 5
#[cfg(test)]
mod tests {

  use paste::paste;

  use core::status::RunResult::SuccessfulRun;
  use testset::PathingMapTest;

  use astar::astar::run;
  use astar::heuristic::Heuristic;

  fn run_test(test_map: &PathingMapTest, heur: Heuristic) {
    let (result, step_data) = run(&test_map.mapstring, heur);
    println!("{}", step_data.pathing_map.as_string());

    let is_solvable = test_map.dist.is_some();

    if is_solvable {
      assert_eq!(result, SuccessfulRun, "Failed to solve map!");

      let dist = test_map.dist.unwrap();

      let loc_data = step_data.loc_data_map.get(&step_data.goal_coord).expect("Goal coord must exist!");
      let trail = loc_data.breadcrumb_opt.as_ref().expect("Trail must exist!").to_sequence();
      let actual = trail.len() - 1;

      assert_eq!(actual, dist as usize, "Wrong solution length!");
    } else {
      assert_ne!(result, SuccessfulRun, "Solved an unsolvable map!");
    }
  }

  macro_rules! gen_map_tests {
    ($($i:literal),* $(,)?) => {
      $(
        paste! {
          #[test]
          fn [<euclidean_ $i>]() {
            run_test(&testset::all_maps()[$i], Heuristic::Euclidean);
          }

          #[test]
          fn [<manhattan_ $i>]() {
            run_test(&testset::all_maps()[$i], Heuristic::Manhattan);
          }
        }
      )*
    };
  }

  gen_map_tests!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
    29, 30, 31, 32, 33, 34, 35, 36, 37, 38
  );
}
