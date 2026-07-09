use std::collections::HashMap;
use std::rc::Rc;

use pf_core::coordinate::{Breadcrumb, Coordinate};
use pf_core::pathingmap::PathingMap;

use super::coordqueue::CoordQueue;
use super::heuristic::Heuristic;

#[derive(Debug)]
pub struct LocationData {
  pub breadcrumb_opt: Option<Rc<Breadcrumb>>,
  pub cost_opt: Option<f64>,
  pub was_visited: bool,
}

#[derive(Debug)]
pub struct StepData {
  pub loc_data_map: HashMap<Coordinate, LocationData>,
  pub max_iters: usize,
  pub pathing_map: PathingMap,
  pub goal_coord: Coordinate,
  pub current_coord: Coordinate,
  pub iters_so_far: usize,
  pub heuristic: Heuristic,
  pub queue: CoordQueue,
}
