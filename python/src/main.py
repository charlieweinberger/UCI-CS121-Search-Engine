import os
import json

import download
import indexer
import save
import similarity

#! /developer/DEV contains the url and html content of each page
DEV_PATH = "../developer/DEV"
#! phonebook.json contains a dictionary with docIDs as
#! keys and paths to files in /developer/DEV as values
PHONEBOOK_PATH = "phonebook.json"
#! /indexes/ contains the batches of inverted indexes
INDEXES_PATH = "indexes"

#! The maximum number of document paths in each batch
BATCH_SIZE = 5000

def load_phonebook() -> dict[str: str]:
    #! Returns the contents of phonebook.json, if phonebook.json exists.
    #! Otherwise, return an empty dictionary.
    if os.path.exists(PHONEBOOK_PATH):
        with open(PHONEBOOK_PATH, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}

#! Acts as a generator for yielding document file paths in /developer/DEV
def get_document_paths():
    for root, _, files in os.walk(DEV_PATH):
        for file in files:
            yield os.path.join(root, file)

def save_phonebook(phonebook: dict[str: str]) -> None:
    #! Write the contents of `phonebook` to phonebook.json
    with open(PHONEBOOK_PATH, "w", encoding="utf-8") as f:
        json.dump(phonebook, f, indent=4)

def save_inverted_index(phonebook: dict[str: str],
                        similarity: similarity.SimilarityDetector,
                        current_batch: list[str], batch_counter: int) -> int:

    #! Create a new inverted index for this branch
    inverted_index = indexer.InvertedIndex(current_doc_id=doc_id_counter)

    #! Process each document in the current batch
    for doc_path in current_batch:
        #! Get the content and url from the document in /developer/DEV
        document = download.Document(doc_path)
        url, content = document.url, document.content
        #! If there is no content, then return early
        if content is None:
            continue
        #! If the content is too similar to previously seen pages,
        #! or if the document is too large, then return early
        sim = similarity.is_duplicate_or_similar(doc_path, content)
        if sim[0] or os.path.getsize(doc_path) > 5 * 1024 * 1024:
            continue
        #! Add the content to the inverted index
        token_count = inverted_index.add_document(content)
        #! Increment the document counter, and add the document path,
        #! token count, and url to the phonebook
        doc_id_counter += 1
        phonebook[str(doc_id_counter)] = [doc_path, token_count, url] #! W rizz

    #! Save the inverted index by writing it to a new file
    save_path = f"{INDEXES_PATH}/batch_{batch_counter}.txt"
    save.save_inverted_index(inverted_index, save_path)

    #! Return the docID counter
    return doc_id_counter

def main() -> None:

    #! The current batch of document paths
    current_batch: list[str] = []

    #! Counters for the current batch and docID
    batch_counter: int = 0
    doc_id_counter: int = 0

    #! TODO
    similarity = similarity.SimilarityDetector()

    #! At the beginning, `phonebook` is an empty dictionary. If we've previously
    #! edited phonebook.json, then we load it's contents into `phonebook`.
    #! By the end of `main()`, we save `phonebook` to phonebook.json.
    phonebook = load_phonebook()

    #! Loop through all files within all folders within /developer/DEV
    for document_path in get_document_paths():
        #! Add the file path to the current batch
        current_batch.append(document_path)
        #! When the batch size is reached...
        if len(current_batch) >= BATCH_SIZE:
            #! ...save the current batch as an invertex index and update `phonebook`...
            doc_id_counter = save_inverted_index(phonebook, similarity,
                                                 current_batch, batch_counter)
            #! ...and then reset the current batch and increment the counter
            current_batch = []
            batch_counter += 1

    #! Process remaining documents in the final batch
    if current_batch:
        doc_id_counter = save_inverted_index(doc_id_counter)

    #! Save the contents of `phoneboook` to phonebook.json
    save_phonebook(phonebook)

if __name__ == "__main__":
    main()