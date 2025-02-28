use std::time;
use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::{file_skip_list, tokenizer::Tokenizer};
use std::fs::File;

pub struct SearchEngine {
    query: String,
    tokens: Vec<String>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn get_query(&mut self) {
        self.query.clear();
        print!("Enter your search query: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut self.query)
            .expect("Failed to read query");
        self.query = self.query.trim().to_string();
        self.tokens = Tokenizer::new().tokenize(&self.query);
    }

    pub fn search(&self) {
        let time = time::Instant::now();
        println!("Searching for: \"{}\"", self.query);
        println!("Tokens: {:?}", self.tokens);
        let mut candidates: HashMap<u16, Candidate> = HashMap::new();
        // two docids shouldnt have two candidates

        for token in &self.tokens {
            // get the first letter of the token to determine which file to read
            if let Some(first_char) = token.chars().next() {
                // go to the file_skip list of the first character
                let skiplist = file_skip_list::FileSkip::read_skip_list(first_char);
                // get which BYTE range it is in between
                let offset_range =
                    file_skip_list::FileSkip::find_skip_entry(&skiplist, token.to_string());
                let file_path = format!("inverted_index/merged/{}.txt", first_char);

                if let Ok(file) = File::open(&file_path) {
                    // get the postings from the file and update the scorings of the candidates
                    let postings =
                        file_skip_list::get_postings_from_offset_range(&file, offset_range, token);

                    // Update candidates with the postings data
                    for single_posting in postings.postings {
                        let candidate = candidates
                            .entry(single_posting.doc_id)
                            .or_insert_with(|| Candidate::new(single_posting.doc_id));
                        candidate.update_score(token, single_posting.term_freq);
                    }
                } else {
                    println!("Warning: Could not open index file for '{}'", first_char);
                }
            }
        }

        // calculate scores for all candidates
        for candidate in candidates.values_mut() {
            candidate.calculate_score(&self.tokens);
        }

        // Convert HashMap into a vector for sorting
        let mut results: Vec<&Candidate> = candidates.values().collect();
        // Sort by score in descending order
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // display results with the biggest scores
        println!("Found {} matching documents", results.len());
        if !results.is_empty() {
            println!("Top results:");
            for (i, candidate) in results.iter().take(10).enumerate() {
                println!(
                    "{}. Document ID: {}, Score: {:.2}",
                    i + 1,
                    candidate.doc_id,
                    candidate.score
                );
            }
        }

        println!("Search took: {}ms", time.elapsed().as_millis());
    }
}

pub struct Candidate {
    pub doc_id: u16,
    pub score: f32,
    pub tokens: HashMap<String, u16>, // currently a boolean query
}

impl Candidate {
    pub fn new(doc_id: u16) -> Self {
        Self {
            doc_id,
            score: 0.0,
            tokens: HashMap::new(),
        }
    }

    pub fn update_score(&mut self, token: &str, count: u16) {
        self.tokens.insert(token.to_string(), count);
    }

    pub fn calculate_score(&mut self, query: &Vec<String>) {
        for token in query {
            if let Some(&count) = self.tokens.get(token) {
                self.score += count as f32;
            }
        }
    }
}

// ? Boolean Candidates, pushing only if every token is present in the candidate
pub fn get_valid_candidates<'a>(
    candidates: Vec<&'a Candidate>,
    query: &'a Vec<String>,
) -> Vec<&'a Candidate> {
    let mut valid_candidates = Vec::new();
    for candidate in candidates {
        let mut valid = true;
        for token in query {
            if !candidate.tokens.contains_key(token) {
                valid = false;
                break;
            }
        }
        if valid {
            valid_candidates.push(candidate);
        }
    }
    valid_candidates
}
