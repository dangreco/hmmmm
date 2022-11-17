# hmmmm 🤔

Yet another aptly-named Hidden-Markov Model library for Rust.



## Examples:

```rust
use hmmmm::{Hmm, algorithms::Viterbi};
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

  let pr_initial: [f64; 4] = [0.25, 0.25, 0.25, 0.25];
  
  let pr_transition: [[f64; 4]; 4] = [
    [90. / 91., 1. / 91., 0., 0.], // ~ 91 days of spring
    [0., 92. / 93., 1. / 93., 0.], // ~ 93 days of summer
    [0., 0., 89. / 90., 1. / 90.], // ~ 90 days of fall
    [1. / 91., 0., 0., 90. / 91.], // ~ 91 days of winter
  ];
  
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