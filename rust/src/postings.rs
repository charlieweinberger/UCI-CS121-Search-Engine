use std::collections::BTreeMap;

use crate::single_posting::Posting;

#[derive(Clone)]
pub struct Postings {
    pub word: String,
    postings: BTreeMap<u32, Posting>,
    skip_list: Vec<SkipList>,
}

#[derive(Clone)]
pub struct SkipList {
    doc_id: u32,
    index: u32,
}

impl Postings {
    pub fn new(term: String) -> Postings {
        Postings {
            word: term,
            postings: BTreeMap::new(),
            skip_list: Vec::new(),
        }
    }

    pub fn push(&mut self, posting: Posting) {
        // Find the position where the new posting should be inserted
        self.postings.insert(posting.doc_id, posting);
    }
    // Tested with using binary heap instead of vec, timing shows that it takes 200 more seconds at a batch size of 10k
    // which makes sense since you have to drain, you can really insert, though I should try a btreemap
    pub fn update_frequency(&mut self, doc_id: u32) {
        if let Some(posting) = self.postings.get_mut(&doc_id) {
            posting.increment_freq();
        } else {
            self.push(Posting::new(doc_id, 1));
        }
    }

    pub fn get_postings(&self) -> &BTreeMap<u32, Posting> {
        &self.postings
    }

    pub fn get_postings_mut(&mut self) -> &mut BTreeMap<u32, Posting> {
        &mut self.postings
    }

    pub fn load_postings(line: &str) -> Result<Postings, &'static str> {
        if line.is_empty() {
            return Err("Empty line");
        }
        let (word, postings_str) = line.split_once(':').unwrap();
        let mut postings = Postings::new(word.to_string());
        for single_posting in postings_str.split(",") {
            let (doc_id, term_frequency) = single_posting.split_once("|").unwrap();
            postings.push(Posting::new(
                doc_id.trim().parse::<u32>().unwrap(),
                term_frequency.trim().parse::<u32>().unwrap(),
            ));
        }
        return Ok(postings);
    }

    pub fn merge(&mut self, other: Postings) {
        if self.word != other.word {
            panic!("Merging two different terms");
        }
        for (doc_id, posting) in other.postings {
            self.postings.insert(doc_id, posting);
        }
    }

    pub fn save_postings(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.word);
        result.push(':');
        for posting in &self.postings {
            let posting = posting.1;
            result.push_str(&format!("{}|{},", posting.doc_id, posting.term_freq));
        }
        result.pop();
        result
    }
}
