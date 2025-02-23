use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}

struct InvertedIndex {
    index: HashMap<String, Vec<Posting>>,
    ordered_keys: Vec<String>,
}
struct Posting {
    doc_id: u32,
    term_freq: u32,
}

impl Posting {
    fn new(doc_id: u32, term_freq: u32) -> Posting {
        Posting { doc_id, term_freq }
    }

    fn increment_freq(&mut self) {
        self.term_freq += 1;
    }
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        InvertedIndex {
            index: HashMap::new(),
            ordered_keys: Vec::new(),
        }
    }

    fn add_term(&mut self, term: &str, doc_id: u32) {
        let posting = self.index.entry(term.to_string()).or_insert(Vec::new());
        if let Some(last_posting) = posting.last_mut() {
            if last_posting.doc_id == doc_id {
                last_posting.increment_freq();
                return;
            }
        }
        posting.push(Posting::new(doc_id, 1));
        self.ordered_keys.push(term.to_string());
    }

    fn get_postings(&self, term: &str) -> Option<&Vec<Posting>> {
        self.index.get(term)
    }

    fn get_ordered_keys(&self) -> &Vec<String> {
        &self.ordered_keys
    }
}
