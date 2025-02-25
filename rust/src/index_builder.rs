use crate::inverted_index;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

const PATH: &str = "../developer/DEV/";
pub const BATCH_SIZE: u16 = 5000; // Define the batch size

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

fn process_document(
    filepath: std::path::PathBuf,
    tx: Sender<(u16, String, String)>,
    id_book: Arc<Mutex<HashMap<u16, (String, String)>>>,
    doc_id: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let content: String = fs::read_to_string(&filepath)?; // error:Error
    let doc: Document = serde_json::from_str(&content)?;
    let text = get_only_text_from_html(&doc.content, doc.encoding);
    tx.send((doc_id, doc.url.clone(), text)).unwrap(); // Error: SendError + sync
    let mut id_book_locked = id_book.lock().unwrap();
    id_book_locked.insert(doc_id, (doc.url, filepath.to_str().unwrap().to_string()));
    Ok(())
}

fn write_batch_to_disk(
    inverted_indexes: &Arc<Mutex<inverted_index::InvertedIndexSplit>>,
    batch_count: usize,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut inverted_indexes_locked = inverted_indexes.lock().unwrap();
    inverted_indexes_locked.write_to_disk(format!(
        "inverted_index/{}",
        batch_count / BATCH_SIZE as usize - 1
    ))?;
    *inverted_indexes_locked = inverted_index::InvertedIndexSplit::new(); // Reset the index
    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut doc_id: u16 = 1;

    // shared between threads
    let id_book: Arc<Mutex<HashMap<u16, (String, String)>>> = Arc::new(Mutex::new(HashMap::new()));
    let inverted_indexes: Arc<Mutex<inverted_index::InvertedIndexSplit>> =
        Arc::new(Mutex::new(inverted_index::InvertedIndexSplit::new()));

    let time = time::Instant::now();

    // https://doc.rust-lang.org/book/ch16-02-message-passing.html once you make an index, send it to the main thread to write to disk
    // Create a channel to send data from threads to the main thread
    let (tx, rx): (
        Sender<(u16, String, String)>,
        Receiver<(u16, String, String)>,
    ) = channel();

    let mut handles = vec![]; // Vector to store thread handles

    for dir_entry in fs::read_dir(PATH)? {
        let dir = dir_entry?;
        //iterate over the files in the directory
        // Will error if dir ever contains a non-directory (a file)
        for file in fs::read_dir(dir.path())? {
            let file: fs::DirEntry = file?;
            // assumption is file is a json file
            let filepath = file.path();
            // each sender, needs a way to send, so clone the sender
            let tx_clone = tx.clone();

            // https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct
            // Clone the Arc to share ownership between threads
            let id_book_clone = Arc::clone(&id_book);

            // Spawn a new thread
            let filepath_clone = filepath.clone();
            let handle = thread::spawn(async move || {
                if let Err(e) = process_document(filepath_clone, tx_clone, id_book_clone, doc_id) {
                    eprintln!("Error processing document: {}", e);
                }
            });
            handles.push(handle);
            doc_id += 1;
        }
    }

    // Drop the original sender to signal the end of sending
    drop(tx);

    // Process documents received from the threads
    let mut batch_count = 0;
    // The loop needs to terminate when all senders are dropped.
    // The `recv()` method returns a `Result`, with `Err` indicating that the channel is closed.
    while let Ok((id, _url, text)) = rx.recv() {
        let mut inverted_indexes_locked = inverted_indexes.lock().unwrap();
        inverted_indexes_locked.add_document(id, &text);
        batch_count += 1;

        // Write to disk if we've processed BATCH_SIZE documents
        if batch_count % BATCH_SIZE as usize == 0 {
            if let Err(e) = write_batch_to_disk(&inverted_indexes, batch_count) {
                eprintln!("Error writing batch to disk: {}", e);
            } else {
                println!("Successfully written batch to disk:");
            }

            println!(
                "Processed {} documents in {} minutes",
                batch_count,
                time.elapsed().as_secs() / 60
            );
        }
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Write final batch if any documents remain
    let inverted_indexes_locked = inverted_indexes.lock().unwrap();
    if batch_count % BATCH_SIZE as usize != 0 {
        if let Err(e) = inverted_indexes_locked.write_to_disk(format!(
            "inverted_index/{}",
            batch_count / BATCH_SIZE as usize
        )) {
            eprintln!("Error writing final batch to disk: {}", e);
        } else {
            println!(
                "Successfully written final batch to disk, total docs: {}",
                doc_id
            );
        }
    }

    // Write id_book to disk
    let id_book_locked = id_book.lock().unwrap();
    if let Ok(serialized) = serde_json::to_string(&*id_book_locked) {
        if let Err(e) = fs::write("id_book.json", serialized) {
            eprintln!("Error writing id_book to file: {}", e);
        }
    }
    Ok(())
}
