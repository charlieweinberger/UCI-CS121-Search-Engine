use serde_json::{json, Value};

use crate::postings::Postings;
use crate::tokenizer::Tokenizer;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::Write,
};

#[allow(dead_code)]
pub struct InvertedIndex {
    index: HashMap<String, Postings>,
    ordered_keys: BinaryHeap<String>,
}

#[allow(dead_code)]
impl InvertedIndex {
    pub fn new() -> InvertedIndex {
        InvertedIndex {
            index: HashMap::new(),
            ordered_keys: BinaryHeap::new(),
        }
    }

    pub fn insert(&mut self, term: String, doc_id: u32) {
        let postings = self.index.entry(term.clone()).or_insert(Postings::new());
        postings.update_frequency(doc_id);
        self.ordered_keys.push(term);
    }

    pub fn get_postings(&self, term: &str) -> Option<&Postings> {
        self.index.get(term)
    }

    pub fn get_ordered_keys(&self) -> Vec<String> {
        self.ordered_keys.iter().cloned().collect()
    }
}

//  *This might also help us with multithreading if we wish to make it multithreaded
// https://nlp.stanford.edu/IR-book/pdf/irbookonlinereading.pdf
// page 76, a-f, g-p, q-z get their own inverted indexes and post in their own files
// lets also make one for 0-9
// lets also make one for special characters
pub struct InvertedIndexSplit {
    pub a_f: InvertedIndex,
    pub g_p: InvertedIndex,
    pub q_z: InvertedIndex,
    pub zero_nine: InvertedIndex,
    pub tokenizer: Tokenizer,
}

impl InvertedIndexSplit {
    pub fn new() -> InvertedIndexSplit {
        InvertedIndexSplit {
            a_f: InvertedIndex::new(),
            g_p: InvertedIndex::new(),
            q_z: InvertedIndex::new(),
            zero_nine: InvertedIndex::new(),
            tokenizer: Tokenizer::new(),
        }
    }

    pub fn add_document(&mut self, doc_id: u32, content: &str) {
        for term in self.tokenizer.tokenize(content) {
            let term = term.to_lowercase();
            let first_char = term.chars().next().unwrap();
            match first_char {
                'a'..='f' => self.a_f.insert(term, doc_id),
                'g'..='p' => self.g_p.insert(term, doc_id),
                'q'..='z' => self.q_z.insert(term, doc_id),
                '0'..='9' => self.zero_nine.insert(term, doc_id),
                _ => {}
            }
        }
    }

    pub fn write_to_disk(&self, location: String) -> std::io::Result<()> {
        // Helper function to convert an InvertedIndex to JSON
        fn index_to_json(index: &InvertedIndex) -> Value {
            let mut entries = Vec::new();
            for term in index.get_ordered_keys() {
                if let Some(postings) = index.get_postings(&term) {
                    let posting_pairs: Vec<_> = postings
                        .get_postings()
                        .iter()
                        .map(|post| vec![post.doc_id, post.term_freq])
                        .collect();

                    entries.push(json!({
                        "term": term,
                        "postings": posting_pairs
                    }));
                }
            }
            json!(entries)
        }

        // Write each index to its own file
        let write_index = |filename: &str, index: &InvertedIndex| -> std::io::Result<()> {
            std::fs::create_dir_all(&location)?;
            let json_data = index_to_json(index);
            let mut file = File::create(format!("{}/{}", location, filename))?;
            let formatted = serde_json::to_string_pretty(&json_data)?;
            file.write_all(formatted.as_bytes())?;
            Ok(())
        };

        write_index("a_f.json", &self.a_f)?;
        write_index("g_p.json", &self.g_p)?;
        write_index("q_z.json", &self.q_z)?;
        write_index("0_9.json", &self.zero_nine)?;

        Ok(())
    }
}

// ? Example:
// a_f.json -> [ {term: "apple", postings: [[1,2], [3,4], []]} ]
// where 1 is the doc id and 2 is the frequency of the term in THAT document
