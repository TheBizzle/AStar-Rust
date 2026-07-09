#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
  Failure,
  Success,
  Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunResult {
  FailedRun,
  SuccessfulRun,
}
