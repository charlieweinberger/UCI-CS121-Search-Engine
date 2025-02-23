use crate::single_posting::Posting;

#[allow(dead_code)]
pub struct Postings {
    postings: Vec<Posting>,
    skip_list: Vec<SkipList>,
}

#[allow(dead_code)]
pub struct SkipList {
    doc_id: u32,
    index: u32,
}

impl Postings {
    pub fn new() -> Postings {
        Postings {
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
}
