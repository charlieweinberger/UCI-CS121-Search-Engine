pub mod file_skip_list;
pub mod index_builder;
pub mod inverted_index;
pub mod lazy_merger;
pub mod postings;
pub mod query;
pub mod single_posting;
pub mod tokenizer;
pub mod id_book;

fn main() {
    // ! BUILD INDEX
    // index_builder::main();
    // ! MERGE BATCHES
    // ! The following code snippet merges the batches of inverted indexes into a single inverted index.
    // lazy_merger::main();

    // ! Comes searching and ranking now

    println!("Welcome to the Search Engine!");
    let mut search_engine = query::SearchEngine::new();
    loop {
        search_engine.get_query();
        search_engine.search();
    }
    // TODO: Implement search and ranking logic using the query
}
