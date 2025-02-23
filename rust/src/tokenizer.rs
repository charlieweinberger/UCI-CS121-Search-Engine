use porter_stemmer::stem;
use regex::Regex;
use std::collections::HashSet;
pub struct Tokenizer {
    pub stopwords: HashSet<String>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        let stopwords = include_str!("../data/stopwords.txt")
            .lines()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();
        Tokenizer { stopwords }
    }
    pub fn tokenize(&self, token: &str) -> Vec<String> {
        let pattern = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
        pattern
            .split(token)
            .map(|s| s.to_lowercase())
            .filter(|s| !self.stop_words(s))
            .map(|s| self.porter_stemmer(&s))
            .collect::<Vec<String>>()
    }

    pub fn porter_stemmer(&self, token: &str) -> String {
        stem(token)
    }

    pub fn stop_words(&self, token: &str) -> bool {
        if token.len() <= 2 {
            return true;
        }
        self.stopwords.contains(token)
    }
}
