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
                    for single_posting in postings.postings {
                        candidate.update_score(single_posting.doc_id, single_posting.term_freq);
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

        let mut candidates = Arc::try_unwrap(candidates).unwrap().into_inner().unwrap();
        candidates.sort_by(|a, b| a.doc_ids.len().cmp(&b.doc_ids.len()));

        let mut boolean_and_candidates: HashMap<&u16, &u16> =
            candidates[0].doc_ids.iter().collect();
        for candidate in candidates.iter().skip(1) {
            boolean_and_candidates.retain(|doc_id, _| candidate.doc_ids.contains_key(doc_id));
        }
        let final_time = time.elapsed().as_millis();
        println!("Search took: {}ms", final_time);
        let mut sorted_candidates: Vec<(&u16, &u16)> = boolean_and_candidates.into_iter().collect();
        sorted_candidates.sort_by(|a, b| b.1.cmp(a.1));

        let mut results = Vec::new();
        for (doc_id, term_freq) in sorted_candidates.iter().take(5) {
            let doc = IDBookElement::get_doc_from_id(**doc_id);
            println!(
                "{}|> {}: {} (Score: {})",
                doc_id,
                doc.url,
                doc.path.display(),
                term_freq
            );
            results.push(doc.url.clone());
        }
        (results, final_time)
    }
}

#[derive(Debug)]
pub struct Candidate {
    pub term: String,
    pub doc_ids: HashMap<u16, u16>, // for each doc_id, the term frequency
}

impl Candidate {
    pub fn new(token: String) -> Self {
        Self {
            term: token,
            doc_ids: HashMap::new(),
        }
    }
    pub fn update_score(&mut self, doc_id: u16, term_freq: u16) {
        self.doc_ids.insert(doc_id, term_freq);
    }
}
