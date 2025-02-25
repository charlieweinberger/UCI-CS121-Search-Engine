pub mod inverted_index;
pub mod postings;
pub mod single_posting;
pub mod tokenizer;
pub mod index_builder;
pub mod lazy_merger;

fn main() {
    // ! BUILD INDEX
    // index_builder::main();
    // ! MERGE BATCHES
    // ! The following code snippet merges the batches of inverted indexes into a single inverted index.
    lazy_merger::main();
}

