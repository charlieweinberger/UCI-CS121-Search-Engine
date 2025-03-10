# combining everything together
import download
import indexer
import save
import os
import json
from similarity import SimilarityDetector

DEV_PATH = "../developer/DEV"

SRC_DIR = os.path.dirname(os.path.abspath(__file__))
PHONEBOOK_PATH = os.path.join(SRC_DIR, "phonebook.json")
INDEXES_PATH = os.path.join(SRC_DIR, "indexes")

def load_phonebook():
    """Loads the phonebook.json if it exists; otherwise, returns an empty dictionary."""
    if os.path.exists(PHONEBOOK_PATH):
        with open(PHONEBOOK_PATH, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}

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
    phonebook = load_phonebook()

    doc_id_counter = 0  # Add a counter to track document IDs
        
    
    def process_document(document_path):
        document = download.Document(document_path)
        return document.content

    def save_inverted_index(doc_id_counter):
        # Create a new inverted index for this branch
        inverted_index = indexer.InvertedIndex(current_doc_id=doc_id_counter)
        # Process documents sequentially
        for doc_path in current_batch:
            content = process_document(doc_path)
            sim = similarity.is_duplicate_or_similar(doc_path, content)
            if content is None or sim[0] or os.path.getsize(doc_path) > 5 * 1024 * 1024:
                continue
            # Skip invalid document
            inverted_index.add_document(content)
            # Add to the phonebook
            doc_id_counter += 1
            phonebook[str(doc_id_counter)] = doc_path

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