# combining everything together
import download
import indexer
import save


DEV_PATH = "../developer/DEV"
INDEXES_PATH = "./indexes"


def main():
    # * Download the documents
    batch_size = 5000
    batch_count = 0
    current_batch: list[str] = []
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
            inverted_index.add_document(content)
            doc_id_counter += 1
        save_path = f"{INDEXES_PATH}/batch_{batch_count}.txt"
        save.save_inverted_index(inverted_index, save_path)
        inverted_index.save_phonebook()

    for document_path in download.generator_files(DEV_PATH):
        current_batch.append(document_path)
        # When batch size is reached, process and save
        if len(current_batch) >= batch_size:
            save_inverted_index(doc_id_counter)
            current_batch = []
            batch_count += 1

    # Process remaining documents in the final batch
    if current_batch:
        save_inverted_index(doc_id_counter)


if __name__ == "__main__":
    main()
