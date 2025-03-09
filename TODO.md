# TODO

- [x] Make Report

  - [x] The number of indexed documents
  - [x] The number of unique words
  - [x] The total size on hard disk in KB

- [ ] Indexer
  - [x] Our Cloud VM doesn't have enough memory to hold the entire index in memory.
  - [x] Offload inverted index to disk at least 3 times during construction
  - [x] Create partial indexes during construction
  - [x] Merge partial indexes at the end
  - [x] Optional: Split merged index into files by term ranges
  - [x] Ensure search reads postings from disk, not memory
- [x] Searcher
- [x] ensure speed is less than 300ms
- [ ] Potentially making the indexing and the querying process parallel/multi-threaded
- [ ] Implement a ranking algorithm
- [ ] Implement a spell checker

- [ ] When making this repository open source I want to make a 2-gram search index along with a 3-gram search index and use those as well since I think they better capture positional information rather than holding positional information in the index itself.
- [ ] Potentially trying to add positions information to the index along with MAYBE an extent list to see if that improves the search speed.
- [ ] Duplication checking and removal
- [ ] PageRank and scraping via paths
- [ ] Make a seperate index which indexes pages with their locations and make it so that if the text is in a important tag you incease the tf idf score signigicantly
- [ ] Add BERT Tokenizer on an experimental branch along wiht testing out tokenizers from things like GPT and so on.
