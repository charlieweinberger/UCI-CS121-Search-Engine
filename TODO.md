# TODO

- [X] Make Report

  - [X] The number of indexed documents
  - [X] The number of unique words
  - [X] The total size on hard disk in KB

- [ ] Indexer
  - [ ] Our Cloud VM doesn't have enough memory to hold the entire index in memory.
  - [X] Offload inverted index to disk at least 3 times during construction
  - [X] Create partial indexes during construction
  - [X] Merge partial indexes at the end
  - [X] Optional: Split merged index into files by term ranges
  - [ ] Ensure search reads postings from disk, not memory
- [ ] Searcher
- [ ] ensure speed is less than 300ms
- [ ] Potentially making the indexing and the querying process parallel/multi-threaded
- [ ] Implement a ranking algorithm
- [ ] Implement a spell checker
