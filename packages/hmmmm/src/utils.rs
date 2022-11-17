use crate::prelude::{Observation, State};

pub fn pr_i<S, F>(seed_function: F) -> [f64; S::COUNT]
where
  S: State,
  F: Fn(S) -> f64,
  [(); S::COUNT]:,
{
  let mut pr = [0.0f64; S::COUNT];

  (0..S::COUNT).for_each(|i| {
    let s = S::from_usize(i);
    pr[i] = seed_function(s);
  });

  pr
}

pub fn pr_t<S, F>(seed_function: F) -> [[f64; S::COUNT]; S::COUNT]
where
  S: State + Copy,
  F: Fn(S, S) -> f64,
  [(); S::COUNT]:,
{
  let mut pr = [[0.0f64; S::COUNT]; S::COUNT];

  (0..S::COUNT).for_each(|i| {
    let a = S::from_usize(i);
    (0..S::COUNT).for_each(|j| {
      let b = S::from_usize(j);
      pr[i][j] = seed_function(a, b);
    })
  });

  pr
}

pub fn pr_e<S, O, F>(seed_function: F) -> [[f64; O::COUNT]; S::COUNT]
where
  S: State + Copy,
  O: Observation + Copy,
  F: Fn(S, O) -> f64,
  [(); S::COUNT]:,
  [(); O::COUNT]:,
{
  let mut pr = [[0.0f64; O::COUNT]; S::COUNT];

  (0..S::COUNT).for_each(|i| {
    let s = S::from_usize(i);
    (0..O::COUNT).for_each(|j| {
      let o = O::from_usize(j);
      pr[i][j] = seed_function(s, o);
    })
  });

  pr
}
