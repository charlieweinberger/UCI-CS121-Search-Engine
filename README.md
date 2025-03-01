# A3

Currently building the Inverted Index

## M1

Goal Building the index:

- Given only HTML files
- Inverted index = token -> key, list of corresponding postings
- Postings = docID, frequency of token in doc (tf-idf score for that document, for MS1 it is just the freq), position of token in doc

Tips:

When designing your inverted index, you will think about the structure
of your posting first.
• You would normally begin by implementing the code to calculate/fetch
the elements which will constitute your posting.
• Modularize. Use scripts/classes that will perform a function or a set of
closely related functions. This helps in keeping track of your progress,
debugging, and also dividing work amongst teammates if you’re in a group.
• We recommend you use GitHub as a mechanism to work with your team
members on this project, but you are not required to do so.

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

The rust version: is incomplete and only makes

### Deliverables

- The number of indexed documents;
- The number of unique words;
- The total size (in KB) of your index on disk.
