# combining everything together
import download
import indexer
import save
import os
import json
from similarity import SimilarityDetector

DEV_PATH = "../developer/DEV"
INDEXES_PATH = "indexes/"
PHONEBOOK_PATH = "phonebook.json"


def save_phonebook(phonebook):
    """Writes the phonebook dictionary to a JSON file."""
    with open(PHONEBOOK_PATH, "w", encoding="utf-8") as f:
        json.dump(phonebook, f, indent=4)


def main():
    # * Download the documents
    batch_size = 5000
    batch_count = 0
    current_batch: list[str] = []
    similarity = SimilarityDetector()
    phonebook = dict()
    doc_id_counter = 1

    def process_document(document_path):
        document = download.Document(document_path)
        return document

    def save_inverted_index(doc_id_counter):
        # Create a new inverted index for this branch
        inverted_index = indexer.InvertedIndex(current_doc_id=doc_id_counter)
        # Process documents sequentially
        for doc_path in current_batch:
            document = process_document(doc_path)
            content = document.content
            url = document.url
            if content is None:
                continue
            sim = similarity.is_duplicate_or_similar(doc_path, content)
            if sim[0] or os.path.getsize(doc_path) > 5 * 1024 * 1024:
                continue
            # Skip invalid document
            length = inverted_index.add_document(content, doc_id_counter)
            phonebook[doc_id_counter] = [doc_path, length, url]
            print([doc_id_counter, doc_path, length, url])
            # Add to the phonebook
            doc_id_counter += 1

        save_path = f"{INDEXES_PATH}/batch_{batch_count}.txt"
        save.save_inverted_index(inverted_index, save_path)
        return doc_id_counter
    for document_path in download.generator_files(DEV_PATH):
        current_batch.append(document_path)
        # When batch size is reached, process and save
        if len(current_batch) >= batch_size:
            doc_id_counter = save_inverted_index(doc_id_counter)
            current_batch = []
            batch_count += 1

    # Process remaining documents in the final batch
    if current_batch:
        doc_id_counter = save_inverted_index(doc_id_counter)
    save_phonebook(phonebook)


if __name__ == "__main__":
    main()
