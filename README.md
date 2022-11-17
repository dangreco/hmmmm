# hmmmm 🤔

Yet another aptly-named Hidden-Markov Model library for Rust.



## Examples:

```rust
use hmmmm::{Hmm, algorithms::Viterbi, utils::*};
use hmmmm_derive::{State, Observation};

#[repr(u8)]
#[derive(State)]
enum Season {
  Spring,
  Summer,
  Fall,
  Winter,
}

#[repr(u8)]
#[derive(Observation)]
enum Weather {
  ClearSky,
  Rain,
  Snow,
  Lightning,
}

pub fn main() {

  let pr_initial: [f64; 4] = pr_i(|season| match season {
    Season::Spring => 91. / 365.,
    Season::Summer => 93. / 365.,
    Season::Fall => 90. / 365.,
    Season::Winter => 91. / 365.,
  });
  
  let pr_transition: [[f64; 4]; 4] = pr_t(|a, b| match (a, b) {
    (Season::Spring, Season::Spring) => 90. / 91.,
    (Season::Spring, Season::Summer) => 1. / 91.,

    (Season::Summer, Season::Summer) => 92. / 93.,
    (Season::Summer, Season::Fall) => 1. / 93.,

    (Season::Fall, Season::Fall) => 89. / 90.,
    (Season::Fall, Season::Winter) => 1. / 90.,

    (Season::Winter, Season::Winter) => 90. / 1.,
    (Season::Winter, Season::Spring) => 1. / 90.,
  });
 
  let pr_emission: [[f64; 4]; 4] = [
    [0.90, 0.08, 0.00, 0.02],
    [0.80, 0.12, 0.00, 0.08],
    [0.78, 0.18, 0.02, 0.02],
    [0.75, 0.05, 0.15, 0.00],
  ];

  let hmm = Hmm::<Season, Weather>::new(
    pr_initial,
    pr_transition,
    pr_emission
  );

  let signal: Vec<Weather> = vec![Weather::Sunny, ......., Weather::Snowy];
  
  let (pr, sequence) = hmm.map_estimate::<Viterbi>(&signal);

}
```