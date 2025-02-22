# TODO

- [ ] Make Report

  - [ ] The number of indexed documents
  - [ ] The number of unique words
  - [ ] The total size on hard disk in KB

- [ ] Indexer
  - [ ] Our Cloud VM doesn't have enough memory to hold the entire index in memory.
  - [ ] Offload inverted index to disk at least 3 times during construction
  - [ ] Create partial indexes during construction
  - [ ] Merge partial indexes at the end
  - [ ] Optional: Split merged index into files by term ranges
  - [ ] Ensure search reads postings from disk, not memory
- [ ] Searcher
- [ ] ensure speed is less than 300ms
