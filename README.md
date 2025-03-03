# A3

Working on querying the documents

## M1

Goal: Develop a search and retrieval component


Developing the Search component
Once you have built the inverted index, you are ready to test document retrieval
with queries. At the very least, the search should be able to deal with boolean
queries: AND only.
If you wish, you can sort the retrieved documents based on tf-idf scoring
(you are not required to do so now, but doing it now may save you time in
the future). This can be done using the cosine similarity method. Feel free to
use a library to compute cosine similarity once you have the term frequencies
and inverse document frequencies (although it should be very easy for you to
write your own implementation). You may also add other weighting/scoring
mechanisms to help refine the search result

### Instructions to run the code

Please run the python version of the code since the rust version is unstable and incomplete!

Make the partial index using:

```bash
cd python
python src/main.py
```

Combine the partial indexes using:

```bash
python src/lazy_merger.py
```

Query with our interface:
```bash
python src/run_queries.py
```


### Deliverables

• the top 5 URLs for each of the queries above
• a picture of your search interface in action
