use std::collections::{BinaryHeap, HashMap};

use crate::postings::Postings;

#[allow(dead_code)]
struct InvertedIndex {
    index: HashMap<String, Postings>,
    ordered_keys: BinaryHeap<String>,
}

#[allow(dead_code)]
impl InvertedIndex {
    fn new() -> InvertedIndex {
        InvertedIndex {
            index: HashMap::new(),
            ordered_keys: BinaryHeap::new(),
        }
    }

    fn insert(&mut self, term: String, doc_id: u32) {
        let postings = self.index.entry(term.clone()).or_insert(Postings::new());
        postings.update_frequency(doc_id);
        self.ordered_keys.push(term);
    }

    fn get_postings(&self, term: &str) -> Option<&Postings> {
        self.index.get(term)
    }

    fn get_ordered_keys(&self) -> Vec<String> {
        self.ordered_keys.iter().cloned().collect()
    }

    fn add_document(&mut self, doc_id: u32, content: &str) {

    }
}
