use rand::distributions::WeightedIndex;
use rand::prelude::*;

use crate::prelude::{MapEstimate, Observation, State};

/// The main hidden Markov model data structure
#[allow(dead_code)]
pub struct Hmm<S, O>
where
  S: State,
  O: Observation,
  [(); S::COUNT]:,
  [(); O::COUNT]:,
{
  _phantom: std::marker::PhantomData<(S, O)>,
  pub pr_initial: [f64; S::COUNT],
  pub pr_transition: [[f64; S::COUNT]; S::COUNT],
  pub pr_emission: [[f64; O::COUNT]; S::COUNT],
}

#[allow(dead_code)]
impl<S, O> Hmm<S, O>
where
  S: State + Copy,
  O: Observation + Copy,
  [(); S::COUNT]:,
  [(); O::COUNT]:,
{
  pub fn new(
    pr_initial: [f64; S::COUNT],
    pr_transition: [[f64; S::COUNT]; S::COUNT],
    pr_emission: [[f64; O::COUNT]; S::COUNT],
  ) -> Self {
    Self {
      _phantom: std::marker::PhantomData,
      pr_initial,
      pr_transition,
      pr_emission,
    }
  }

  pub fn map_estimate<A>(&self, signal: &Vec<O>) -> (f64, Vec<S>)
  where
    A: MapEstimate<S, O>,
  {
    A::map_estimate(&self, signal)
  }

  pub fn sample(&self, length: usize) -> Vec<O> {
    let mut observations = Vec::new();

    if length == 0 {
      return observations;
    }

    let mut rng = thread_rng();

    let initial: WeightedIndex<f64> = WeightedIndex::new(self.pr_initial).unwrap();

    let transition: Vec<WeightedIndex<f64>> = (0..S::COUNT)
      .map(|i| WeightedIndex::new(self.pr_transition[i]).unwrap())
      .collect();

    let emission: Vec<WeightedIndex<f64>> = (0..S::COUNT)
      .map(|i| WeightedIndex::new(self.pr_emission[i]).unwrap())
      .collect();

    let mut state: S = S::from_usize(initial.sample(&mut rng));

    for _ in 0..length {
      let i: usize = state.as_usize();
      observations.push(O::from_usize(emission[i].sample(&mut rng)));
      state = S::from_usize(transition[i].sample(&mut rng));
    }

    observations
  }
}
