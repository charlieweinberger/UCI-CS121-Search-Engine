use std::collections::HashMap;
use std::{fs, time};
use crate::inverted_index;
use serde::Deserialize;

const PATH: &str = "../developer/DEV/";
pub const BATCH_SIZE: u32 = 5000; // Define the batch size

pub fn main() {
    let mut doc_id = 1;
    let mut id_book: HashMap<u32, String> = HashMap::new();
    let mut inverted_indexes = inverted_index::InvertedIndexSplit::new();
    let time = time::Instant::now();
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
                        println!("Processed {} documents in {} seconds", doc_id, time.elapsed().as_secs());
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
