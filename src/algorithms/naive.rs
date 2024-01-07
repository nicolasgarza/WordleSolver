use crate::{Guess, Guesser, DICTIONARY};
use std::collections::HashMap;

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
    pub fn new() -> Self {
        Naive {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (word, count) = line
                    .split_once(' ')
                    .expect("every line is word + space + frequency");
                let count: usize = count.parse().expect("every count is a number");
                (word, count)
            })),
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word));
        }

        let total_count: usize = self.remaining.iter().map(|(_, &c)| c).sum();

        let mut best: Option<Candidate> = None;
        for (&word, &count) in &self.remaining {
            // [word1, word2, ..., wordn]
            // 
            // for word_i, the goodness is the sum of the goodness of each possible pattern we
            // _might_ see as a result of guessing it, multiplied by the likelihood of that pattern
            // occuring.
            //
            //
            // TODO: how do we compute this?
            // - SUM_i pi_i * log_2(pi_i)
            let p_word = count as f64 / total_count as f64;
            let goodness = 0.0 - (p_word * p_word.log2());
            if let Some(c) = best {
                // is this one better?
                if goodness > c.goodness {
                    best = Some(Candidate {
                        word,
                        count,
                        goodness,
                    });
                }
            } else {
                best = Some(Candidate {
                    word,
                    count,
                    goodness,
                });
            }
        }
        best.unwrap().word.to_string()
    }
}
