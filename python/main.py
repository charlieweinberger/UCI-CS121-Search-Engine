# combining everything together

import time
import json
import download
import indexer
import save

# ! current path of the folder for me is /developer/DEV, change for your self
CURRENT_PATH = "../developer/DEV"


def main():
    # * Download the documents
    batch_size = 5000
    batch_count = 0
    current_batch = []
    doc_id_counter = 0  # Add a counter to track document IDs

    def process_document(document_path):
        document = download.Document(document_path)
        return document.content

    for document_path in download.generator_files(CURRENT_PATH):
        current_batch.append(document_path)
        # When batch size is reached, process and save
        if len(current_batch) >= batch_size:
            # Create a new inverted index for this batch
            InvertedIndex = indexer.InvertedIndex(
                current_doc_id=doc_id_counter)  # Pass the starting doc_id

            # Process documents sequentially
            for doc_path in current_batch:
                content = process_document(doc_path)
                InvertedIndex.add_document(content)
                doc_id_counter += 1

            save_path = f"indexes/batch_{batch_count}.txt"
            save.save_inverted_index(InvertedIndex, save_path)

            current_batch = []
            batch_count += 1

    # Process remaining documents in the final batch
    if current_batch:
        InvertedIndex = indexer.InvertedIndex(current_doc_id=doc_id_counter)
        for doc_path in current_batch:
            content = process_document(doc_path)
            InvertedIndex.add_document(content)
            doc_id_counter += 1
        save_path = f"indexes/batch_{batch_count}.json"
        save.save_inverted_index(InvertedIndex, save_path)


if __name__ == "__main__":
    main()
