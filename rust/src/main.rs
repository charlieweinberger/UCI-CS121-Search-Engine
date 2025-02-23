use serde::Deserialize;
use std::collections::HashMap;

pub mod inverted_index;
pub mod postings;
pub mod single_posting;
pub mod tokenizer;

const PATH: &str = "../developer/DEV/";
fn main() {
    let mut doc_id = 1;
    // doc id to url just incase we need to retrieve the url
    let mut id_book: HashMap<u32, String> = HashMap::new();
    let mut inverted_indexes_count = 0;
    for dir_entry in std::fs::read_dir(PATH).unwrap() {
        let dir = dir_entry.unwrap();
        let mut inverted_indexes = inverted_index::InvertedIndexSplit::new();
        if dir.path().is_dir() {
            for file in std::fs::read_dir(dir.path()).unwrap() {
                let file = file.unwrap();
                if file.path().extension().unwrap_or_default() == "json" {
                    let content = std::fs::read_to_string(file.path()).unwrap();
                    let doc: Document = serde_json::from_str(&content).unwrap();
                    id_book.insert(doc_id, doc.url.clone());
                    inverted_indexes.add_document(doc_id, &doc.content);
                    doc_id += 1;
                }
            }
        }
        let write_result = inverted_indexes.write_to_disk(format!("inverted_index/{}", inverted_indexes_count));
        inverted_indexes_count += 1;
        match write_result {
            Ok(_) => println!("Successfully written to disk"),
            Err(e) => println!("Error writing to disk: {}", e),
        }
        if let Ok(serialized) = serde_json::to_string(&id_book) {
            if let Err(e) = std::fs::write("id_book.json", serialized) {
                println!("Error writing id_book to file: {}", e);
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Document {
    url: String,
    content: String,
}
