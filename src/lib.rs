pub mod algorithms;

pub fn play<G: Guesser>(answer: &str, guesser: G) {
    // play six rounds where it invokes guesser each round
    let mut history = Vec::new();
    for i in 0.. {}
}

pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Grey
    Wrong,
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}
