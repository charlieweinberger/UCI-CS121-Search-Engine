use postings::Postings;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, Seek, Write};

pub mod inverted_index;
pub mod postings;
pub mod single_posting;
pub mod tokenizer;

const PATH: &str = "../developer/DEV/";
const BATCH_SIZE: u32 = 10000; // Define the batch size

fn main() {
    let mut doc_id = 1;
    let mut id_book: HashMap<u32, String> = HashMap::new();
    let mut inverted_indexes = inverted_index::InvertedIndexSplit::new();

    for dir_entry in fs::read_dir(PATH).unwrap() {
        let dir = dir_entry.unwrap();
        if dir.path().is_dir() {
            for file in fs::read_dir(dir.path()).unwrap() {
                let file = file.unwrap();
                if file.path().extension().unwrap_or_default() == "json" {
                    let content = fs::read_to_string(file.path()).unwrap();
                    let mut doc: Document = serde_json::from_str(&content).unwrap();
                    doc.content = get_only_text_from_html(&doc.content, doc.encoding);

                    // Process single document immediately
                    id_book.insert(doc_id, doc.url.clone());
                    inverted_indexes.add_document(doc_id, &doc.content);

                    // Write to disk if we've processed BATCH_SIZE documents
                    if doc_id % BATCH_SIZE == 0 {
                        match inverted_indexes
                            .write_to_disk(format!("inverted_index/{}", doc_id / BATCH_SIZE - 1))
                        {
                            Ok(_) => println!("Successfully written batch to disk"),
                            Err(e) => println!("Error writing to disk: {}", e),
                        }
                        inverted_indexes = inverted_index::InvertedIndexSplit::new();
                    }
                    doc_id += 1;
                }
            }
        }
    }

    // Write final batch if any documents remain
    if doc_id % BATCH_SIZE != 1 {
        if let Err(e) =
            inverted_indexes.write_to_disk(format!("inverted_index/{}", doc_id / BATCH_SIZE))
        {
            println!("Error writing final batch to disk: {}", e);
        }
    }

    // Write id_book to disk
    if let Ok(serialized) = serde_json::to_string(&id_book) {
        if let Err(e) = fs::write("id_book.json", serialized) {
            println!("Error writing id_book to file: {}", e);
        }
    }

    // ! MERGE BATCHES
    // ! The following code snippet merges the batches of inverted indexes into a single inverted index.
    // Create directory for merged index if it doesn't exist
    fs::create_dir_all("inverted_index/merged").unwrap_or_default();
    // Merge all batches into a single file
    let mut readers = Vec::new();
    let mut lines = Vec::new();

    // Open all files
    for i in 0..((doc_id - 1) / BATCH_SIZE + 1) {
        for words in ["a_f", "g_p", "q_z", "0_9"].iter() {
            if let Ok(file) = fs::File::open(format!("inverted_index/{}/{}.txt", i, words)) {
                // like in python, this is creating a buffer  reader for each file and appending it
                // (other solution is storing each file and reading using a trait and custom reader from my own buffer but thats to complex for now)
                readers.push(std::io::BufReader::new(file));
                // for each reader there is the line buffer
                lines.push(String::new());
            }
        }
    }

    let mut merged_file = fs::File::create("inverted_index/merged/complete.txt").unwrap();
    let mut active_indices: Vec<usize> = (0..readers.len()).collect();
    let mut needs_new_line = vec![true; readers.len()]; // Cache for whether a reader needs a new line

    while !active_indices.is_empty() {
        // Read lines and get valid postings
        let mut current_postings = Vec::new();

        for &idx in &active_indices {
            if needs_new_line[idx] {
                lines[idx].clear();
                // read_line returns the number of bytes read, 0 if EOF
                if readers[idx].read_line(&mut lines[idx]).unwrap() > 0 {
                    if let Ok(posting) = Postings::load_postings(&lines[idx]) {
                        current_postings.push((idx, posting));
                    }
                }
            } else if let Ok(posting) = Postings::load_postings(&lines[idx]) {
                current_postings.push((idx, posting));
            }
        }
        if current_postings.is_empty() {
            break;
        }

        current_postings.sort_by(|a, b| a.1.word.cmp(&b.1.word));
        let smallest_term = current_postings[0].1.word.clone();

        // merge all postings with smallest term
        let mut merged = Postings::new(smallest_term.clone());
        let mut to_remove = Vec::new();

        for (reader_idx, posting) in current_postings {
            if posting.word == smallest_term {
                merged.merge(posting);
                needs_new_line[reader_idx] = true;
                lines[reader_idx].clear();
                if readers[reader_idx]
                    .read_line(&mut lines[reader_idx])
                    .unwrap()
                    == 0
                {
                    to_remove.push(reader_idx);
                }
            } else {
                needs_new_line[reader_idx] = false;
            }
        }

        // Write merged posting
        merged_file
            .write_all(merged.save_postings().as_bytes())
            .unwrap();
        merged_file.write_all(b"\n").unwrap();

        // Remove completed readers
        active_indices.retain(|&idx| !to_remove.contains(&idx));
    }
}

#[derive(Debug, Deserialize)]
struct Document {
    url: String,
    content: String,
    encoding: String,
}

fn get_only_text_from_html(content: &str, encoding: String) -> String {
    let ascii_content = if encoding.to_lowercase().contains("ascii") {
        content.chars().filter(|c| c.is_ascii()).collect::<String>()
    } else {
        match encoding.to_lowercase().as_str() {
            enc if enc.contains("utf-8") => content.to_string(),
            enc if enc.contains("iso-8859") => content.to_string(), // ISO-8859 is already ASCII compatible
            _ => content.chars().filter(|c| c.is_ascii()).collect::<String>(), // fallback to ASCII filtering
        }
    };

    let document = scraper::Html::parse_document(&ascii_content);
    let selector = scraper::Selector::parse("body")
        .unwrap_or_else(|_| scraper::Selector::parse("html").unwrap());

    if let Some(body) = document.select(&selector).next() {
        body.text()
            .collect::<String>()
            .chars()
            .filter(|c| c.is_ascii())
            .collect()
    } else {
        String::new()
    }
}
