use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

use pf_core::coordinate::{Breadcrumb, Coordinate};

#[derive(Debug, Default)]
pub struct CoordQueue {
  heap: BinaryHeap<PrioBundle<MiniLoc>>,
}

impl CoordQueue {
  #[must_use]
  pub const fn new() -> Self {
    Self { heap: BinaryHeap::new() }
  }

  pub fn pop(&mut self) -> Option<PrioBundle<MiniLoc>> {
    self.heap.pop()
  }

  pub fn push(&mut self, priority: f64, item: MiniLoc) {
    self.heap.push(PrioBundle { priority, item });
  }
}

#[derive(Debug)]
pub struct PrioBundle<T> {
  pub priority: f64,
  pub item: T,
}

impl<T> Ord for PrioBundle<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    other.priority.partial_cmp(&self.priority).unwrap()
  }
}

impl<T> PartialOrd for PrioBundle<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T> PartialEq for PrioBundle<T> {
  fn eq(&self, other: &Self) -> bool {
    self.priority.total_cmp(&other.priority) == Ordering::Equal
  }
}

impl<T> Eq for PrioBundle<T> {}

#[derive(Debug)]
pub struct MiniLoc {
  pub breadcrumb: Rc<Breadcrumb>,
  pub coord: Coordinate,
  pub cost: f64,
}
