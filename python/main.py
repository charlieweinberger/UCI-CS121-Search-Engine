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
    InvertedIndex = indexer.InvertedIndex()
    for document_path in download.generator_files(CURRENT_PATH):
        document = download.Document(document_path)
        if not document.verify_content_HTML():
            continue
        InvertedIndex.add_document(document.content)
    save_path = "index.json"
    save.save_inverted_index(InvertedIndex, save_path)


if __name__ == "__main__":
    main()