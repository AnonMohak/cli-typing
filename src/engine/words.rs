use rand::{Rng, seq::IndexedRandom};

const ENGLISH: &str = include_str!("../../assets/words/english.txt");

/// The built-in English list: the 200 most common words.
pub fn english() -> Vec<&'static str> {
    ENGLISH
        .lines()
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .collect()
}

/// Picks random words from a list and never gives the same word two times in a row.
pub struct WordGen<R> {
    list: Vec<&'static str>,
    rng: R,
    last: Option<&'static str>,
}

impl<R: Rng> WordGen<R> {
    pub fn new(list: Vec<&'static str>, rng: R) -> Self {
        assert!(!list.is_empty(), "word list is empty");
        Self {
            list,
            rng,
            last: None,
        }
    }

    pub fn next_word(&mut self) -> &'static str {
        loop {
            let word = *self.list.choose(&mut self.rng).expect("list is not empty");
            if self.list.len() == 1 || Some(word) != self.last {
                self.last = Some(word);
                return word;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn english_list_has_200_unique_words() {
        let list = english();
        assert_eq!(list.len(), 200);
        let mut sorted = list.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 200);
        assert!(
            list.iter()
                .all(|w| w.chars().all(|c| c.is_ascii_lowercase()))
        );
    }

    #[test]
    fn never_repeats_a_word_in_a_row() {
        let mut generator = WordGen::new(vec!["a", "b"], StdRng::seed_from_u64(1));
        let mut prev = generator.next_word();
        for _ in 0..100 {
            let next = generator.next_word();
            assert_ne!(prev, next);
            prev = next;
        }
    }
}
