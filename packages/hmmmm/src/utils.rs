use crate::prelude::{Observation, State};

/// Seeds an initial probability matrix with the given closure
/// 
/// #### Example:
/// 
/// ````rust
/// let pr_initial: [f64; 2] = pr_i(|cond| match cond {
///   Condition::Healthy => 0.6,
///   Condition::Fever => 0.4,
/// });
/// ````
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

/// Seeds a transition probability matrix with the given closure
///
/// #### Example:
/// 
/// ````rust
/// let pr_transition: [[f64; 2]; 2] = pr_t(|a, b| match (a, b) {
///   (Condition::Healthy, Condition::Healthy) => 0.7,
///   (Condition::Healthy, Condition::Fever) => 0.3,
///   (Condition::Fever, Condition::Healthy) => 0.4,
///   (Condition::Fever, Condition::Fever) => 0.6,
/// });
/// ````
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

/// Seeds an emission probability matrix with the given closure
/// 
/// #### Example:
/// 
/// ````rust
/// let pr_emission: [[f64; 3]; 2] = pr_e(|cond, feel| match (cond, feel) {
///   (Condition::Healthy, Feeling::Normal) => 0.5,
///   (Condition::Healthy, Feeling::Cold) => 0.4,
///   (Condition::Healthy, Feeling::Dizzy) => 0.1,
///   (Condition::Fever, Feeling::Normal) => 0.1,
///   (Condition::Fever, Feeling::Cold) => 0.3,
///   (Condition::Fever, Feeling::Dizzy) => 0.6,
/// });
/// ````
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
