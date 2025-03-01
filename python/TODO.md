# TODO LIST FOR PYTHON

## Vocabulary

Postings : (docID, frequency of that word in the docid)
Dictionary: word -> Postings

- [X] Let's for now use the `python` indexes, so build the index using main, and before running lazy merger, make it so that lazy merger merges the postings in the dictionaries but into files starting from (0-9) and (a-z) .txt like a long one of them (if you run the rust version you might be able to see what i mean) (lazy_merger.py)
- [X] Within lazy_merger, replaced final_index with a new dir to merged_indexes (lazy_merger.py)
- [ ] Make sure that the phonebook.json made also lines up with those indexes and load that in memory if possible since you are going to be using that for DOCID to URL mapping
- [X] Get User input from console initially and make it so that it tokenizes the word using our tokenizer class (query.py)
- [X] after tokenizing all your queries, make a candidates hashmap of type [docid, (tokens matched)] (query.py)
- [X] Make a class/function to take a token and search for it in the inverted index (so like take the first letter of each query and then in that file search for the token) and retrieve the postings list (query.py)
- [X] Do that for all tokens and check if all tokens are matched and then print all the results (query.py)

