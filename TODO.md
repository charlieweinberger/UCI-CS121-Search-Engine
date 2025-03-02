# TODO

- [X] Make Report

  - [X] The number of indexed documents
  - [X] The number of unique words
  - [X] The total size on hard disk in KB

- [ ] Indexer
  - [X] Our Cloud VM doesn't have enough memory to hold the entire index in memory.
  - [X] Offload inverted index to disk at least 3 times during construction
  - [X] Create partial indexes during construction
  - [X] Merge partial indexes at the end
  - [X] Optional: Split merged index into files by term ranges
  - [X] Ensure search reads postings from disk, not memory
- [ ] Searcher
- [X] ensure speed is less than 300ms
- [ ] Potentially making the indexing and the querying process parallel/multi-threaded
- [ ] Implement a ranking algorithm
- [ ] Implement a spell checker

- [ ] When making this repository open source I want to make a 2-gram search index along with a 3-gram search index and use those as well since I think they better capture positional information rather than holding positional information in the index itself.
- [ ] Potentially trying to add positions information to the index along with MAYBE an extent list to see if that improves the search speed.
