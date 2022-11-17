use hmmmm::*;

#[repr(u8)]
#[derive(State, PartialEq, Debug)]
enum Condition {
  Healthy,
  Fever,
}

#[repr(u8)]
#[derive(Observation)]
enum Feeling {
  Normal,
  Cold,
  Dizzy,
}

fn main() {
  let hmm = Hmm::<Condition, Feeling>::new(
    [0.6, 0.4],
    [[0.7, 0.3], [0.4, 0.6]],
    [[0.5, 0.4, 0.1], [0.1, 0.3, 0.6]],
  );

  let signal = vec![Feeling::Normal, Feeling::Cold, Feeling::Dizzy];
  let (probability, sequence) = hmm.map_estimate::<algorithms::Viterbi>(&signal);

  assert_eq!(probability.floor(), -7.0);
  assert_eq!(
    sequence,
    vec![Condition::Healthy, Condition::Healthy, Condition::Fever]
  );
}
