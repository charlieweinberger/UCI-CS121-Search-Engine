use std::f32::consts::E;

use crate::single_posting::Posting;

#[derive(Clone)]
pub struct Postings {
    pub word: String,
    postings: Vec<Posting>,
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
            postings: Vec::new(),
            skip_list: Vec::new(),
        }
    }

    pub fn push(&mut self, posting: Posting) {
        // Find the position where the new posting should be inserted
        let insert_pos = self
            .postings
            .binary_search_by_key(&posting.doc_id, |p| p.doc_id)
            .unwrap_or_else(|pos| pos);
        // Insert the posting at the correct position
        self.postings.insert(insert_pos, posting);
    }

    pub fn update_frequency(&mut self, doc_id: u32) {
        let pos = self.postings.binary_search_by_key(&doc_id, |p| p.doc_id);
        match pos {
            Ok(pos) => {
                self.postings[pos].increment_freq();
            }
            Err(_) => {
                self.push(Posting::new(doc_id, 1));
            }
        }
    }

    pub fn get_postings(&self) -> &Vec<Posting> {
        &self.postings
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
        let mut i = 0;
        let mut j = 0;
        while i < self.postings.len() && j < other.postings.len() {
            if self.postings[i].doc_id < other.postings[j].doc_id {
                i += 1;
            } else if self.postings[i].doc_id > other.postings[j].doc_id {
                self.postings.insert(i, other.postings[j].clone());
                i += 1;
                j += 1;
            }
        }
        while j < other.postings.len() {
            self.postings.push(other.postings[j].clone());
            j += 1;
        }
    }

    pub fn save_postings(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.word);
        result.push(':');
        for posting in &self.postings {
            result.push_str(&format!("{}|{},", posting.doc_id, posting.term_freq));
        }
        result.pop();
        result
    }
}
