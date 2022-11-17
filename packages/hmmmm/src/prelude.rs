use crate::Hmm;

pub trait State {
  const COUNT: usize;

  fn as_usize(&self) -> usize;
  fn from_usize(value: usize) -> Self;
}

pub trait Observation {
  const COUNT: usize;

  fn as_usize(&self) -> usize;
  fn from_usize(value: usize) -> Self;
}

pub trait MapEstimate<S, O>
where
  S: State,
  O: Observation,
  [(); S::COUNT]:,
  [(); O::COUNT]:,
{
  fn map_estimate(hmm: &Hmm<S, O>, signal: &Vec<O>) -> (f64, Vec<S>);
}
