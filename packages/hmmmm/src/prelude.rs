use crate::Hmm;

pub trait State {
  const COUNT: usize;

  fn as_usize(&self) -> usize;
  fn from_usize(value: usize) -> Self;

  fn as_u8(&self) -> u8;
  fn from_u8(value: u8) -> Self;
}

pub trait Observation {
  const COUNT: usize;

  fn as_usize(&self) -> usize;
  fn from_usize(value: usize) -> Self;

  fn as_u8(&self) -> u8;
  fn from_u8(value: u8) -> Self;
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
