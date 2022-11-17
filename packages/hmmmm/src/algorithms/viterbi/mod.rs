use crate::{
  prelude::{MapEstimate, Observation, State},
  Hmm,
};

pub struct Viterbi;

impl<S, O> MapEstimate<S, O> for Viterbi
where
  S: State,
  O: Observation,
  [(); S::COUNT]:,
  [(); O::COUNT]:,
{
  fn map_estimate(hmm: &Hmm<S, O>, signal: &Vec<O>) -> (f64, Vec<S>) {
    let mut probabilities = vec![vec![0.0; signal.len()]; S::COUNT];
    let mut traces = vec![vec![0; signal.len()]; S::COUNT];

    for i in 0..S::COUNT {
      probabilities[i][0] =
        hmm.pr_initial[i].log2() + hmm.pr_emission[i][signal[0].as_usize()].log2();
      traces[i][0] = 0;
    }

    for j in 1..signal.len() {
      for i in 0..S::COUNT {
        let emission = hmm.pr_emission[i][signal[j].as_usize()].log2();
        let (k, p) = (0..S::COUNT)
          .map(|k| {
            (
              k,
              probabilities[k][j - 1] + hmm.pr_transition[k][i].log2() + emission,
            )
          })
          .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
          .unwrap();

        probabilities[i][j] = p;
        traces[i][j] = k;
      }
    }

    let (k, p) = (0..S::COUNT)
      .map(|k| (k, probabilities[k][signal.len() - 1]))
      .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
      .unwrap();

    let mut sequence = Vec::new();
    let mut k = k;

    for j in 0..signal.len() {
      sequence.insert(0, S::from_usize(k));
      k = traces[k][j];
    }

    (p, sequence)
  }
}

#[cfg(test)]
mod tests {
  use approx::assert_relative_eq;

  use crate::{
    algorithms::Viterbi,
    prelude::{Observation, State},
    Hmm,
  };

  #[test]
  fn test_1() {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[allow(dead_code)]
    enum S {
      Healthy,
      Fever,
    }

    impl State for S {
      const COUNT: usize = 2;

      fn as_usize(&self) -> usize {
        (*self as u8) as usize
      }

      fn from_usize(value: usize) -> Self {
        [Self::Healthy, Self::Fever][value]
      }
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    enum O {
      Normal,
      Cold,
      Dizzy,
    }

    impl Observation for O {
      const COUNT: usize = 3;

      fn as_usize(&self) -> usize {
        (*self as u8) as usize
      }

      fn from_usize(value: usize) -> Self {
        [Self::Normal, Self::Cold, Self::Dizzy][value]
      }
    }

    let hmm = Hmm::<S, O>::new(
      [0.6, 0.4],
      [[0.7, 0.3], [0.4, 0.6]],
      [[0.5, 0.4, 0.1], [0.1, 0.3, 0.6]],
    );

    let signal = vec![O::Normal, O::Cold, O::Dizzy];

    let (p, sequence) = hmm.map_estimate::<Viterbi>(&signal);

    assert_eq!(sequence, vec![S::Healthy, S::Healthy, S::Fever]);
    assert_relative_eq!(p, -6.04739805, epsilon = 0.000001)
  }

  #[test]
  fn test_2() {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    #[allow(dead_code)]
    enum Season {
      Spring,
      Summer,
      Fall,
      Winter,
    }

    impl State for Season {
      const COUNT: usize = 4;

      fn as_usize(&self) -> usize {
        (*self as u8) as usize
      }

      fn from_usize(value: usize) -> Self {
        [Self::Spring, Self::Summer, Self::Fall, Self::Winter][value]
      }
    }

    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    #[allow(dead_code)]
    enum Weather {
      ClearSky,
      Rain,
      Snow,
      Lightning,
    }

    impl Observation for Weather {
      const COUNT: usize = 4;

      fn as_usize(&self) -> usize {
        (*self as u8) as usize
      }

      fn from_usize(value: usize) -> Self {
        [Self::ClearSky, Self::Rain, Self::Snow, Self::Lightning][value]
      }
    }

    let pr_initial: [f64; 4] = [0.25, 0.25, 0.25, 0.25];

    let pr_transition: [[f64; 4]; 4] = [
      [90. / 91., 1. / 91., 0., 0.], // ~ 91 days of spring
      [0., 92. / 93., 1. / 93., 0.], // ~ 93 days of summer
      [0., 0., 89. / 90., 1. / 90.], // ~ 90 days of fall
      [1. / 91., 0., 0., 90. / 9.1], // ~ 91 days of winter
    ];

    let pr_emission: [[f64; 4]; 4] = [
      [0.90, 0.08, 0.00, 0.02],
      [0.80, 0.12, 0.00, 0.08],
      [0.78, 0.18, 0.02, 0.02],
      [0.75, 0.05, 0.15, 0.00],
    ];

    let hmm = Hmm::<Season, Weather>::new(pr_initial, pr_transition, pr_emission);

    let signal = hmm.sample(365);
    let (pr, sequence) = hmm.map_estimate::<Viterbi>(&signal);

    assert!(!pr.is_nan());
    assert_eq!(sequence.len(), 365);
  }
}
