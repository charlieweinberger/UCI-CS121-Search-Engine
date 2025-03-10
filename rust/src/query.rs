use std::time;
use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::id_book::IDBookElement;
use crate::{file_skip_list, tokenizer::Tokenizer};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;

pub const TOTAL_DOCUMENT_COUNT: u16 = 46843;

pub struct SearchEngine {
    query: String,
    tokens: Vec<String>,
    skiplists: Arc<Vec<Vec<file_skip_list::FileSkip>>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        let mut skiplists = Vec::new();
        for i in 0..=9 {
            let skiplist = file_skip_list::FileSkip::read_skip_list((b'0' + i) as char);
            skiplists.push(skiplist);
        }
        for i in 0..26 {
            let skiplist = file_skip_list::FileSkip::read_skip_list((b'a' + i) as char);
            skiplists.push(skiplist);
        }
        Self {
            query: String::new(),
            tokens: Vec::new(),
            skiplists: Arc::new(skiplists),
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

    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.tokens = Tokenizer::new().tokenize(&self.query);
    }

    pub fn search(&self) -> (Vec<String>, u128) {
        let time = time::Instant::now();
        println!("Searching for: \"{}\"", self.query);
        println!("Tokens: {:?}", self.tokens);

        // This will be shared across threads for adding candidates
        let candidates = Arc::new(Mutex::new(Vec::with_capacity(self.tokens.len())));
        let mut handles = vec![];

        for token in self.tokens.iter() {
            let candidates = Arc::clone(&candidates);
            let skiplists = Arc::clone(&self.skiplists);
            let token = token.clone();

            let handle = thread::spawn(move || {
                let first_char = token.chars().next().unwrap();
                let first_char_index = if first_char.is_ascii_digit() {
                    (first_char as u8 - b'0') as usize
                } else {
                    (first_char as u8 - b'a') as usize + 10
                };

                let offset_range =
                    file_skip_list::FileSkip::find_skip_entry(&skiplists[first_char_index], &token);

                let file_path = format!("inverted_index/merged/{}.txt", first_char);

                let mut candidate = Candidate::new(token.to_string());
                if let Ok(file) = File::open(&file_path) {
                    let postings =
                        file_skip_list::get_postings_from_offset_range(&file, offset_range, &token);
                    let posting_length = if token.len() <= 2 {
                        println!("Warning: Token '{}' is too short", token);
                        TOTAL_DOCUMENT_COUNT - 100
                    } else {
                        postings.postings.len() as u16
                    };
                    for single_posting in postings.postings {
                        let score = scoring_tf_idf(single_posting.term_freq, posting_length);
                        candidate.update_score(single_posting.doc_id, score);
                    }
                } else {
                    println!("Warning: Could not open index file for '{}'", first_char);
                }
                let mut candidates = candidates.lock().unwrap();
                candidates.push(candidate);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        if self.tokens.len() == 0 {
            return (Vec::new(), 0);
        }

        let mut candidates = Arc::try_unwrap(candidates).unwrap().into_inner().unwrap();
        if candidates.len() == 0 {
            return (Vec::new(), 0);
        }
        candidates.sort_by(|a: &Candidate, b: &Candidate| a.doc_ids.len().cmp(&b.doc_ids.len()));
        let mut base: HashMap<u16, f64> = candidates[0].doc_ids.clone();
        for i in 1..candidates.len() {
            let next = &candidates[i].doc_ids;
            base.retain(|k, _| next.contains_key(k));
        }

        let mut all_candidates: HashMap<u16, f64> = base;
        for candidate in candidates.iter() {
            for (doc_id, score) in candidate.doc_ids.iter() {
                all_candidates
                    .entry(*doc_id)
                    .and_modify(|s| *s += score)
                    .or_insert(*score);
            }
        }

        let mut sorted_candidates: Vec<(&u16, &f64)> = all_candidates.iter().collect();
        sorted_candidates.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        let mut results = Vec::new();
        let final_time = time.elapsed().as_millis();
        println!("Search took: {}ms", final_time);
        for (doc_id, score) in sorted_candidates.iter().take(10) {
            let doc = IDBookElement::get_doc_from_id(**doc_id);
            println!(
                "{}|> {}: {} (Score: {})",
                doc_id,
                doc.url,
                doc.path.display(),
                score
            );
            results.push(doc.url.clone());
        }
        (results, final_time)
    }
}

#[derive(Debug)]
pub struct Candidate {
    pub term: String,
    pub doc_ids: HashMap<u16, f64>, // for each doc_id, the tfidf score
}

impl Candidate {
    pub fn new(token: String) -> Self {
        Self {
            term: token,
            doc_ids: HashMap::new(),
        }
    }
    pub fn update_score(&mut self, doc_id: u16, score: f64) {
        self.doc_ids.insert(doc_id, score);
    }
}

pub fn scoring_tf_idf(term_freq: u16, posting_length: u16) -> f64 {
    let tf: f64 = f64::log2(term_freq as f64) + 1.0;
    let idf: f64 = f64::log2(TOTAL_DOCUMENT_COUNT as f64 / posting_length as f64);
    tf * idf
}
