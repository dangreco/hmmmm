# hmmmm 🤔

Yet another aptly-named Hidden-Markov Model library for Rust.


### **Important Note:** This crate depends on the feature `generic_const_exprs`. For now, usage of this crate is limited to use with the `nightly` Rust channel. 


## Usage


### States + Observations

To create a `Hmm`, we first need two enums corresponding to the
states and observations of the hidden Markov model. This can be done
by using the `#[derive(State)]` and `#[derive(Observation)]` proc macros in the `hmmmm_derive` crate: 

```rust
#[repr(u8)]
#[derive(State)]
enum Condition {
  Healthy,
}

#[repr(u8)]
#[derive(Observation)]
enum Feeling {
  Normal,
  Cold,
  Dizzy,
}
```

Note that the attribute `repr(u8)` is required for both enums
definitions, as we need to have the enums be: **a)** as small as
possible for efficient computation and **b)** easily cast between
itself and `u8` / `usize`. The derive macros will throw a compilation
error if this attribute is not present on the enum.

### Probabilities

A `Hmm` requires 3 different probability matrices:

1. Initial (state) probabilities - `[f64; N]`
2. Transition probabilities - `[[f64; N]; N]`
3. Emission probabilities - `[[f64; M]; N]`

...where `N` is the number of possible states and `O` is the
number of possible observations.

These can be:

- Manualy declared, e.g.:

```rust                  
let pr_initial: [f64; 2] = [0.6, 0.4]; // [Healthy, Fever]
```
- Declared via the methods in `hmmmm::utils`, e.g.:
  
```rust
let pr_initial: [f64; 2] = pr_i(|cond| match cond {
  Condition::Healthy => 0.6,
  Condition::Fever => 0.4,
});
```

### Constructing the HMM

Now that we have our states, observations, and corresponding 
probability matrices we can construct our `Hmm`:

```rust
let pr_initial: [f64; 2] = [0.6, 0.4];
let pr_transition: [[f64; 2]; 2] = [
  [0.7, 0.3],
  [0.4, 0.6]
];
let pr_emission: [[f64; 3]; 2] = [
  [0.5, 0.4, 0.1],
  [0.1, 0.3, 0.6]
]

let hmm = Hmm::<Condition, Feeling>::new(
  pr_initial,
  pr_transition,
  pr_emission
);
```

## Algorithms

### Viterbi
 
[Wikipedia](https://en.wikipedia.org/wiki/Viterbi_algorithm)

The Viterbi algorithm obtains the maximum a posteriori estimate 
(MAP) of the most likely sequence of hidden states given an 
input sequence of observations.

**Input:** the sequence of observations - `&Vec<O>`

**Output:** a tuple containing the log (base 2) probability of the MAP hidden state sequence and the corresponding hidden state sequence - `(f64, Vec<S>)`

**Example:**
```rust
/* Using the HMM `hmm` from the previous section */

let signal = vec![Feeling::Normal, Feeling::Cold, Feeling::Dizzy];
let (pr, sequence) = hmm.map_estimate::<Viterbi>(&signal);

/*

pr = −6.04739805022
sequence = [Condition::Healthy, Condition::Healthy, Condition::Healthy]

*/
```