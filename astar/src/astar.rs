use std::collections::HashMap;
use std::rc::Rc;

use core::coordinate::Breadcrumb::{Crumb, Source};
use core::coordinate::Coordinate;
use core::interpreter::PathingMapString;
use core::pathingmap::PathingMap;
use core::status::RunResult::{self, FailedRun, SuccessfulRun};

use super::coordqueue::{CoordQueue, MiniLoc};
use super::heuristic::Heuristic;
use super::stepdata::{LocationData, StepData};

#[must_use]
pub fn run(pms: &PathingMapString, heuristic: Heuristic) -> (RunResult, StepData) {
  fn helper(step_data: &mut StepData) -> RunResult {
    let is_primed = prime_next_step(step_data);
    if step_data.goal_coord == step_data.current_coord {
      draw_results(step_data);
      SuccessfulRun
    } else if !is_primed || step_data.iters_so_far >= step_data.max_iters {
      draw_results(step_data);
      FailedRun
    } else {
      for neighbor in step_data.pathing_map.neighbors_of(step_data.current_coord) {
        let msg1 = &format!("Neighbor has to be within the map: {neighbor:?}");
        if !step_data.loc_data_map.get(&neighbor).expect(msg1).was_visited {
          enqueue_neighbor(neighbor, step_data);
        }
      }

      let current_coord = step_data.current_coord;
      let msg2 = &format!("Current coord has to be within the map: {current_coord:?}");
      step_data.loc_data_map.get_mut(&step_data.current_coord).expect(msg2).was_visited = true;
      step_data.iters_so_far += 1;
      helper(step_data)
    }
  }
  let mut state = gen_initial_state(pms, heuristic);
  let result = helper(&mut state);
  (result, state)
}

fn gen_initial_state(pms: &PathingMapString, heuristic: Heuristic) -> StepData {
  let pmd = pms.as_pmd();

  let self_breadcrumb = Rc::new(Source { coord: pmd.start });
  let self_loc = MiniLoc { breadcrumb: Rc::clone(&self_breadcrumb), coord: pmd.start, cost: 0.0 };

  let mut queue = CoordQueue::new();
  queue.push(0.0, self_loc);

  let mut loc_datas: HashMap<_, _> = pmd
    .grid
    .map
    .keys()
    .map(|k| {
      let value = LocationData { breadcrumb_opt: None, cost_opt: None, was_visited: false };
      (*k, value)
    })
    .collect();

  let loc_data = LocationData {
    breadcrumb_opt: Some(Rc::clone(&self_breadcrumb)),
    cost_opt: Some(0.0 + 0.0), // I just wanted `rustfmt` to split this into several lines
    was_visited: false,
  };
  loc_datas.insert(pmd.start, loc_data);

  let pmap = PathingMap { grid: pmd.grid };

  StepData {
    loc_data_map: loc_datas,
    max_iters: pmap.height() * pmap.width(),
    pathing_map: pmap,
    goal_coord: pmd.goal,
    current_coord: pmd.start,
    iters_so_far: 0,
    heuristic,
    queue,
  }
}

fn prime_next_step(step_data: &mut StepData) -> bool {
  if let Some(next_thing) = step_data.queue.pop() {
    let next_item = next_thing.item;
    let msg = &format!("Queued item's coord must exist on map: {next_item:?}");
    let next_data = step_data.loc_data_map.get_mut(&next_item.coord).expect(msg);
    if next_data.was_visited {
      prime_next_step(step_data)
    } else {
      next_data.breadcrumb_opt = Some(next_item.breadcrumb);
      next_data.cost_opt = Some(next_item.cost);
      step_data.current_coord = next_item.coord;
      true
    }
  } else {
    false
  }
}

fn enqueue_neighbor(neighbor: Coordinate, step_data: &mut StepData) {
  let current_coord = step_data.current_coord;
  let msg1 = &format!("Current coord must exist on map: {current_coord:?}");
  let current_loc = step_data.loc_data_map.get(&current_coord).expect(msg1);

  let msg2 = &format!("Current location must have a registered cost by now: {current_loc:?}");
  let new_cost = current_loc.cost_opt.as_ref().expect(msg2) + 1.0;

  let msg3 = &format!("Neighbor must be a real location: {neighbor:?}");
  let cost_opt = step_data.loc_data_map.get(&neighbor).expect(msg3).cost_opt;

  let msg4 = &format!("Neighbor must have a registered cost by now: {neighbor:?}");
  if cost_opt.is_none() || new_cost < cost_opt.expect(msg4) {
    let msg5 = &format!("A breadcrumb trail must have a trail of its own: {current_loc:?}");
    let h = step_data.heuristic.clone().eval(step_data.goal_coord, neighbor);
    let from = current_loc.breadcrumb_opt.as_ref().expect(msg5);
    let breadcrumb = Rc::new(Crumb { to: neighbor, from: Rc::clone(from) });
    let mini_loc = MiniLoc { breadcrumb, coord: neighbor, cost: new_cost };
    step_data.queue.push(new_cost + h, mini_loc);
  }
}

fn draw_results(step_data: &mut StepData) {
  let queries: Vec<Coordinate> = step_data
    .loc_data_map
    .iter()
    .filter_map(|(coord, loc_data)| {
      if loc_data.was_visited {
        Some(*coord)
      } else {
        None
      }
    })
    .collect();

  step_data.pathing_map.insert_query(queries);

  if step_data.goal_coord == step_data.current_coord {
    let goal_coord = step_data.goal_coord;
    let msg1 = &format!("Goal coord must exist on map: {goal_coord:?}");
    let goal_data = step_data.loc_data_map.get(&goal_coord).expect(msg1);
    let msg2 = &format!("We reached the goal, so this should have had a breadcrumb: {goal_coord:?}");
    let coords = goal_data.breadcrumb_opt.as_ref().expect(msg2).to_sequence();
    step_data.pathing_map.insert_query(coords);
  }
}
