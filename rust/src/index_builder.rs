use crate::inverted_index;
use serde::Deserialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
const PATH: &str = "../developer/DEV/";
pub const BATCH_SIZE: u16 = 5000; // Define the batch size
pub const IDBOOK_PATH: &str = "inverted_index/id_book.txt";
const LINE_BYTE_SIZE: usize = 350;

#[derive(Debug, Deserialize)]
struct Document {
    url: String,
    content: String,
    encoding: String,
}

fn get_only_text_from_html(content: &str, encoding: String) -> String {
    let ascii_content: String = if encoding.to_lowercase().contains("ascii") {
        content.chars().filter(|c| c.is_ascii()).collect::<String>()
    } else {
        match encoding.to_lowercase().as_str() {
            enc if enc.contains("utf-8") => content.to_string(),
            enc if enc.contains("iso-8859") => content.to_string(),
            _ => content.chars().filter(|c| c.is_ascii()).collect::<String>(),
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

fn process_file(file_path: PathBuf, tx_clone: Sender<(u16, String, String, String)>, doc_id: u16) {
    let content: String = fs::read_to_string(&file_path).unwrap();
    let doc: Document = serde_json::from_str(&content).unwrap();
    let text = get_only_text_from_html(&doc.content, doc.encoding);
    // Send the processed document data to the main thread
    tx_clone
        .send((
            doc_id,
            doc.url.clone(),
            text,
            file_path.to_str().unwrap().to_string(),
        ))
        .unwrap();
}

pub fn main() {
    let mut doc_id: u16 = 0;
    // shared between threads
    let inverted_indexes: Arc<Mutex<inverted_index::InvertedIndexSplit>> =
        Arc::new(Mutex::new(inverted_index::InvertedIndexSplit::new()));
    let time = time::Instant::now();
    File::create(IDBOOK_PATH).unwrap();
    // https://doc.rust-lang.org/book/ch16-02-message-passing.html
    // once you make an index, send it to the main thread to write to disk
    // Create a channel to send data from threads to the main thread
    let (tx, rx): (
        Sender<(u16, String, String, String)>,
        Receiver<(u16, String, String, String)>,
    ) = channel();

    let mut handles = vec![]; // Vector to store thread handles

    for dir_entry in fs::read_dir(PATH).unwrap() {
        let dir = dir_entry.unwrap(); // Iterate over the files in the directory
                                      // Will error if dir ever contains a non-directory (a file)
        for file in fs::read_dir(dir.path()).unwrap() {
            doc_id += 1;
            let file: fs::DirEntry = file.unwrap(); // assumption is file is a json file
            let filepath = file.path();

            // each sender, needs a way to send, so clone the sender
            let tx_clone = tx.clone();

            // https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct
            // Clone the Arc to share ownership between threads

            let handle = thread::spawn(move || {
                process_file(filepath, tx_clone, doc_id);
            });
            handles.push(handle);
        }
    }

    // Drop the original sender to signal the end of sending
    drop(tx);

    // Process documents received from the threads
    let mut batch_count = 0;
    let mut id_book_batch: Vec<(u16, String, String)> = Vec::new(); // Accumulate batch here

    // The loop needs to terminate when all senders are dropped.
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(IDBOOK_PATH)
        .unwrap();
    // The `recv()` method returns a `Result`, with `Err` indicating that the channel is closed.
    while let Ok((id, url, text, path)) = rx.recv() {
        let mut inverted_indexes_locked = inverted_indexes.lock().unwrap();
        inverted_indexes_locked.add_document(id, &text);

        id_book_batch.push((id, url, String::new())); // Store id and url for id_book

        batch_count += 1;

        // Write to disk if we've processed BATCH_SIZE documents
        if batch_count % BATCH_SIZE as usize == 0 {
            // Sort the batch by doc_id
            id_book_batch.sort_by_key(|&(id, _, _)| id);
            // Write the batch to id_book.txt
            for (id, url, _) in &id_book_batch {
                if let Err(e) = writeln!(file, "{}: {} | {}", id, url, path) {
                    println!("Error writing line for doc_id {}: {}", id, e);
                }
            }

            id_book_batch.clear(); // Clear the batch

            match inverted_indexes_locked.write_to_disk(format!(
                "inverted_index/{}",
                batch_count / BATCH_SIZE as usize - 1
            )) {
                Ok(_) => println!("Successfully written batch to disk:"),
                Err(e) => println!("Error writing to disk: {}", e),
            }
            *inverted_indexes_locked = inverted_index::InvertedIndexSplit::new(); // Reset the index

            println!(
                "Processed {} documents in {} minutes",
                batch_count,
                time.elapsed().as_secs() / 60
            );
        }
    }

    // Complete all threads before continuing to the main thread
    for handle in handles {
        handle.join().unwrap();
    }

    // Write final batch if any documents remain
    let inverted_indexes_locked = inverted_indexes.lock().unwrap();
    if batch_count % BATCH_SIZE as usize != 0 {
        // Sort the remaining batch
        id_book_batch.sort_by_key(|&(id, _, _)| id);

        // Write the remaining batch to id_book.txt
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(IDBOOK_PATH)
            .unwrap();

        for (id, url, path) in &id_book_batch {
            let mut string_to_write = format!("{}: {} | {}", id, url, path);
            // Adjust string length to match LINE_BYTE_SIZE (including newline)
            if string_to_write.len() >= LINE_BYTE_SIZE - 1 {
                string_to_write.truncate(LINE_BYTE_SIZE - 1); // Leave room for newline
            } else {
                // Pad with spaces to reach LINE_BYTE_SIZE - 1 (space for newline)
                string_to_write.extend(
                    std::iter::repeat(' ').take(LINE_BYTE_SIZE - 1 - string_to_write.len()),
                );
            }
            string_to_write.push('\n');
            if let Err(e) = writeln!(file, "{}", string_to_write) {
                println!("Error writing line for doc_id {}: {}", id, e);
            }
        }

        if let Err(e) = inverted_indexes_locked.write_to_disk(format!(
            "inverted_index/{}",
            batch_count / BATCH_SIZE as usize
        )) {
            println!("Error writing final batch to disk: {}", e);
        } else {
            println!(
                "Successfully written final batch to disk, total docs: {}",
                doc_id
            );
        }
    }

    // No need to write the id_book at the end, it's written in batches
    println!("Finished processing all documents.");
}
