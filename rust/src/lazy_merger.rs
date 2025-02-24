use std::fs;
use std::io::{BufRead, BufReader, LineWriter, Write};
use crate::postings::Postings;
use crate::index_builder::BATCH_SIZE;


pub fn main() {
    fs::create_dir_all("inverted_index/merged").unwrap_or_default();
    let mut readers = Vec::new();
    let doc_id = 55393;
    let batch_count = (doc_id - 1) / BATCH_SIZE + 1;
    let word_ranges = ["0_9", "a_f", "g_p", "q_z"]; // Reordered to process numbers first
    let merged_file = fs::File::create("inverted_index/merged/complete.txt").unwrap();
    let mut final_file_appender = LineWriter::new(merged_file);

    for &words in word_ranges.iter() {
        readers.clear();
        readers.reserve(batch_count as usize);
        for i in 0..batch_count {
            if let Ok(file) = fs::File::open(format!("inverted_index/{}/{}.txt", i, words)) {
                readers.push((BufReader::new(file), String::new()));
            }
        }

        let mut lines: Vec<String> = readers
            .iter_mut()
            .map(|(reader, _)| {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap_or(0);
                line
            })
            .collect();

        let mut active_indices: Vec<usize> = (0..readers.len()).collect();

        while !active_indices.is_empty() {
            // Filter out empty lines (end of file)
            let current_indices: Vec<usize> = active_indices
                .iter()
                .filter(|&i| !lines[*i].trim().is_empty())
                .cloned()
                .collect();

            if current_indices.is_empty() {
                break;
            }

            // get all valid postings
            let postings: Vec<Postings> = current_indices
                .iter()
                .map(|&i| Postings::load_postings(&lines[i]).unwrap())
                .collect();

            // get the smallest key
            let smallest = get_smallest_key(&postings);
            let mut new_posting = Postings::new(smallest.clone());

            // Track which readers need to move to next line
            let mut to_advance: Vec<usize> = Vec::new();
            for &idx in &current_indices {
                let posting = Postings::load_postings(&lines[idx]).unwrap();
                if posting.word == smallest {
                    new_posting.merge(posting);
                    to_advance.push(idx);
                }
            }

            final_file_appender
                .write_all((new_posting.save_postings() + "\n").as_bytes())
                .unwrap();

            // Move to next line for relevant readers
            for idx in to_advance {
                let mut line = String::new();
                let bytes_read = readers[idx].0.read_line(&mut line).unwrap_or(0);
                lines[idx] = line;
                if bytes_read == 0 {
                    // Reader reached end
                    active_indices.retain(|&x| x != idx);
                }
            }
        }
    }
}

fn get_smallest_key(postings: &Vec<Postings>) -> String {
    let mut smallest = postings[0].word.clone();
    for posting in postings {
        if posting.word < smallest {
            smallest = posting.word.clone();
        }
    }
    smallest
}

